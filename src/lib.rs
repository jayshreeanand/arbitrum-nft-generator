#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, FixedBytes, U256, Uint},
    prelude::*,
    evm,
};
use stylus_sdk::alloy_sol_types::sol;
use std::fmt;
use std::fmt::Write;
use stylus_sdk::storage::{StorageAddress, StorageU32};

// Add static buffers for SVG and JSON data
static mut SVG_BUFFER: [u8; 16384] = [0; 16384];  // 16KB buffer for SVG
static mut JSON_BUFFER: [u8; 8192] = [0; 8192];   // 8KB buffer for JSON

sol! {
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event NFTMinted(uint256 indexed tokenId, bytes32 txHash);
}

#[entrypoint]
#[storage]
pub struct Contract {
    owner: StorageAddress,
    token_counter: StorageU32,
}

#[public]
impl Contract {
    pub fn constructor(&mut self) {
        self.token_counter.set(Uint::<32, 1>::from(0u32));
    }

    pub fn supports_interface(&self, interface: FixedBytes<4>) -> bool {
        let interface_slice_array: [u8; 4] = interface.as_slice().try_into().unwrap();
        let id = u32::from_be_bytes(interface_slice_array);

        id == 0x01ffc9a7 || // ERC-165
        id == 0x80ac58cd || // ERC-721
        id == 0x5b5e139f    // ERC-721Metadata
    }

    pub fn mint(&mut self, to: Address) -> U256 {
        let token_id = U256::from(self.token_counter.get());
        self.owner.set(to);
        self.token_counter.set(self.token_counter.get() + Uint::<32, 1>::from(1u32));
        
        // Emit events
        evm::log(Transfer {
            from: Address::ZERO,
            to,
            tokenId: token_id,
        });
        
        evm::log(NFTMinted {
            tokenId: token_id,
            txHash: evm::tx_hash(),
        });

        token_id
    }

    pub fn symbol(&self) -> String {
        "RNFT".to_string()
    }

    pub fn name(&self) -> String {
        "Random Art NFT".to_string()
    }

    pub fn balance_of(&self, owner: Address) -> U256 {
        if owner == self.owner.get() {
            U256::from(1)
        } else {
            U256::from(0)
        }
    }

    pub fn owner_of(&self, token_id: U256) -> Result<Address, Vec<u8>> {
        if token_id >= U256::from(self.token_counter.get()) {
            return Err("Token does not exist".as_bytes().to_vec());
        }

        let owner = self.owner.get();
        if owner == Address::ZERO {
            return Err("Token not minted".as_bytes().to_vec());
        }

        Ok(owner)
    }

    #[selector(name = "tokenURI")]
    pub fn token_uri(&self, token_id: U256) -> String {
        // Get transaction details for randomization
        let tx_hash = evm::tx_hash();
        let tx_hash_bytes = tx_hash.as_bytes();

        struct BufferWriter {
            buf: &'static mut [u8],
            pos: usize,
        }

        impl BufferWriter {
            fn new(buf: &'static mut [u8]) -> Self {
                Self { buf, pos: 0 }
            }
        }

        impl Write for BufferWriter {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                let bytes = s.as_bytes();
                if self.pos + bytes.len() > self.buf.len() {
                    return Err(fmt::Error);
                }
                for &b in bytes {
                    self.buf[self.pos] = b;
                    self.pos += 1;
                }
                Ok(())
            }
        }

        unsafe {
            let svg_buf = &mut SVG_BUFFER;
            let mut svg_writer = BufferWriter::new(svg_buf);

            // Calculate dimensions
            let width = 500;
            let height = 500;
            let grid_size = 5;
            let cell_size = width / grid_size;

            // Write SVG header with a background
            let background_color = Self::get_random_color(tx_hash_bytes, 0);
            let _ = write!(
                svg_writer,
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\"><rect width=\"100%\" height=\"100%\" fill=\"{bg}\"/>",
                w = width,
                h = height,
                bg = background_color
            );

            // Generate random shapes based on transaction hash
            for i in 0..grid_size {
                for j in 0..grid_size {
                    let x = j * cell_size + cell_size / 2;
                    let y = i * cell_size + cell_size / 2;
                    let shape = Self::get_random_shape(
                        tx_hash_bytes,
                        i * grid_size + j,
                        x,
                        y,
                        cell_size
                    );
                    let _ = write!(svg_writer, "{}", shape);
                }
            }

            // Add connecting lines
            for i in 0..10 {
                let x1 = tx_hash_bytes[i % tx_hash_bytes.len()] as usize % width;
                let y1 = tx_hash_bytes[(i + 1) % tx_hash_bytes.len()] as usize % height;
                let x2 = tx_hash_bytes[(i + 2) % tx_hash_bytes.len()] as usize % width;
                let y2 = tx_hash_bytes[(i + 3) % tx_hash_bytes.len()] as usize % height;
                let line_color = Self::get_random_color(tx_hash_bytes, i);
                let _ = write!(
                    svg_writer,
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\" opacity=\"0.5\"/>",
                    x1, y1, x2, y2, line_color
                );
            }

            let _ = write!(svg_writer, "</svg>");

            // Get the final SVG string and base64 encode it
            let svg_pos = svg_writer.pos;
            drop(svg_writer);
            let svg_base64 = Self::base64_encode(&SVG_BUFFER[..svg_pos]);
            let svg_uri = format!("data:image/svg+xml;base64,{}", svg_base64);

            // Create the JSON metadata
            let json_buf = &mut JSON_BUFFER;
            let mut json_writer = BufferWriter::new(json_buf);

            let _ = write!(
                json_writer,
                "{{\"name\":\"Random Art #{}\",\"description\":\"A unique piece of generative art created from transaction {}\",\"image\":\"{}\"}}",
                token_id,
                hex::encode(tx_hash_bytes),
                svg_uri
            );

            // Base64 encode the entire JSON and return as data URI
            let json_pos = json_writer.pos;
            drop(json_writer);
            let json_base64 = Self::base64_encode(&JSON_BUFFER[..json_pos]);

            format!("data:application/json;base64,{}", json_base64)
        }
    }
}

impl Contract {
    fn get_random_color(seed: &[u8], index: usize) -> String {
        let byte = seed[index % seed.len()];
        format!("#{:02x}{:02x}{:02x}", 
            byte.wrapping_mul(7),
            byte.wrapping_mul(13),
            byte.wrapping_mul(17)
        )
    }

    fn get_random_shape(seed: &[u8], index: usize, x: usize, y: usize, size: usize) -> String {
        let shape_type = seed[index % seed.len()] % 3;
        let size_third = size / 3;
        match shape_type {
            0 => format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" opacity=\"0.7\"/>",
                x, y, size_third, Self::get_random_color(seed, index + 1)
            ),
            1 => format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" opacity=\"0.7\"/>",
                x - size_third, y - size_third, size/2, size/2, Self::get_random_color(seed, index + 2)
            ),
            _ => format!(
                "<polygon points=\"{},{} {},{} {},{}\" fill=\"{}\" opacity=\"0.7\"/>",
                x, y - size_third,
                x - size_third, y + size_third,
                x + size_third, y + size_third,
                Self::get_random_color(seed, index + 3)
            ),
        }
    }

    fn base64_encode(input: &[u8]) -> String {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::with_capacity((input.len() + 2) / 3 * 4);

        for chunk in input.chunks(3) {
            let b = match chunk.len() {
                3 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32),
                2 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8),
                1 => (chunk[0] as u32) << 16,
                _ => unreachable!(),
            };

            result.push(ALPHABET[(b >> 18 & 0x3F) as usize] as char);
            result.push(ALPHABET[(b >> 12 & 0x3F) as usize] as char);

            if chunk.len() > 1 {
                result.push(ALPHABET[(b >> 6 & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }

            if chunk.len() > 2 {
                result.push(ALPHABET[(b & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
        }

        result
    }
}