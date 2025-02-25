#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, FixedBytes, U256, Uint},
    prelude::*,
};
use stylus_sdk::alloy_sol_types::sol;
use std::fmt::{self, Write};
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
    #[payable]
    fn constructor(&mut self) {
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
        
        // Get block number and convert to bytes32
        let block_num = self.vm().block_number();
        let mut pseudo_hash = [0u8; 32];
        pseudo_hash[24..32].copy_from_slice(&block_num.to_be_bytes());
        let pseudo_hash = FixedBytes::<32>::from(pseudo_hash);
        
        // Emit events
        log(self.vm(), Transfer {
            from: Address::ZERO,
            to,
            tokenId: token_id,
        });
        
        log(self.vm(), NFTMinted {
            tokenId: token_id,
            txHash: pseudo_hash,
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
        // Create a seed array from token ID
        let mut seed = [0u8; 32];
        
        // Convert token_id to bytes and use them directly
        let token_bytes: [u8; 32] = token_id.to_be_bytes();
        
        // Create a more varied seed using token_id bytes
        for i in 0..32 {
            // Mix the token bytes with different prime numbers to create variation
            seed[i] = (
                (token_bytes[i] as u16 * 167 +  // Use prime numbers
                (i as u16 * 191) +              // Different prime for position
                ((i * 7 + 13) % 256) as u16     // Additional variation
                ) % 256
            ) as u8;
        }

        // Additional mixing pass to increase randomness
        for i in 0..31 {
            seed[i] ^= seed[i + 1];
        }

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
            let grid_size = 6;
            let cell_size = width / grid_size;

            // Write SVG header with a random background color
            let bg_color = Self::get_random_color(&seed, 31);
            let _ = write!(
                svg_writer,
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\"><rect width=\"100%\" height=\"100%\" fill=\"{bg}\"/>",
                w = width,
                h = height,
                bg = bg_color
            );

            // Generate shapes in a grid
            for i in 0..grid_size {
                for j in 0..grid_size {
                    let x = (j * cell_size) + (cell_size / 2);
                    let y = (i * cell_size) + (cell_size / 2);
                    let shape_size = cell_size * 4 / 5;
                    let shape = Self::get_random_shape(
                        &seed,
                        i * grid_size + j,
                        x,
                        y,
                        shape_size
                    );
                    let _ = write!(svg_writer, "{}", shape);
                }
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
                "{{\"name\":\"Random Art #{}\",\"description\":\"A unique piece of generative art\",\"image\":\"{}\"}}",
                token_id,
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
        let base = (index * 17 + 13) % seed.len();  // Use prime numbers for better distribution
        
        // Generate base color components
        let r = ((seed[base] as u16 * 7 + seed[(base + 1) % seed.len()] as u16 * 13) % 256) as u8;
        let g = ((seed[(base + 2) % seed.len()] as u16 * 11 + seed[(base + 3) % seed.len()] as u16 * 17) % 256) as u8;
        let b = ((seed[(base + 4) % seed.len()] as u16 * 19 + seed[(base + 5) % seed.len()] as u16 * 23) % 256) as u8;
        
        // Ensure colors are vibrant by boosting the lowest components
        let min_val = 60; // Minimum color component value
        let r = r.max(min_val);
        let g = g.max(min_val);
        let b = b.max(min_val);
        
        // Ensure at least one component is very bright
        let max_component = r.max(g).max(b);
        if max_component < 180 {
            let brightest = index % 3;
            match brightest {
                0 => format!("#{:02x}{:02x}{:02x}", 255, g, b),
                1 => format!("#{:02x}{:02x}{:02x}", r, 255, b),
                _ => format!("#{:02x}{:02x}{:02x}", r, g, 255),
            }
        } else {
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        }
    }

    fn get_random_shape(seed: &[u8], index: usize, x: usize, y: usize, size: usize) -> String {
        let base = index % seed.len();
        // Use more bits from the seed for shape selection
        let shape_type = ((seed[base] as u16 * seed[(base + 7) % seed.len()] as u16) % 5) as usize;
        let size_half = size / 2;
        
        // Get multiple colors with different indices for more variation
        let color1 = Self::get_random_color(seed, index * 5 + 1);
        let color2 = Self::get_random_color(seed, index * 7 + 3);
        
        // More varied rotation
        let rotation = ((seed[(base + 3) % seed.len()] as u32 * 
                        seed[(base + 7) % seed.len()] as u32 +
                        seed[(base + 11) % seed.len()] as u32) % 360) as u32;
        
        match shape_type {
            0 => {
                // Circle with gradient stroke
                let radius = size_half * 
                    (50 + (seed[base] as usize % 50)) / 100;
                format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"4\"/>",
                    x, y, radius, color1, color2
                )
            },
            1 => {
                // Rectangle with gradient
                let width = size * (50 + (seed[(base + 1) % seed.len()] as usize % 50)) / 100;
                let height = size * (50 + (seed[(base + 2) % seed.len()] as usize % 50)) / 100;
                format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"3\" transform=\"rotate({} {} {})\"/>",
                    x - width/2, y - height/2, width, height, color1, color2, rotation, x, y
                )
            },
            2 => {
                // Star-like shape
                let points = format!(
                    "{},{} {},{} {},{} {},{} {},{}",
                    x, y - size_half,
                    x + size_half/2, y - size_half/4,
                    x + size_half, y,
                    x + size_half/2, y + size_half/2,
                    x, y + size_half
                );
                format!(
                    "<polygon points=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\" transform=\"rotate({} {} {})\"/>",
                    points, color1, color2, rotation, x, y
                )
            },
            3 => {
                // Cross
                format!(
                    "<g transform=\"rotate({} {} {})\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/></g>",
                    rotation, x, y,
                    x - size/6, y - size_half, size/3, size, color1,
                    x - size_half, y - size/6, size, size/3, color2
                )
            },
            _ => {
                // Diamond with three colors
                format!(
                    "<path d=\"M {} {} L {} {} L {} {} L {} {} Z\" fill=\"{}\" stroke=\"{}\" stroke-width=\"3\" transform=\"rotate({} {} {})\"/>",
                    x, y - size_half,
                    x + size_half, y,
                    x, y + size_half,
                    x - size_half, y,
                    color1, color2,
                    rotation, x, y
                )
            }
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