// proxy-server.js
const express = require('express');
const cors = require('cors');
const { createProxyMiddleware } = require('http-proxy-middleware');

const app = express();

// Configure CORS
app.use(cors({
    origin: 'http://localhost:9001',
    methods: ['GET', 'POST', 'OPTIONS'],
    allowedHeaders: ['Content-Type']
}));

// Create proxy
app.use('/', createProxyMiddleware({
    target: 'http://localhost:8547',
    changeOrigin: true,
    onProxyRes: function (proxyRes) {
        proxyRes.headers['Access-Control-Allow-Origin'] = 'http://localhost:9001';
    }
}));

app.listen(3000, () => {
    console.log('Proxy server running on port 3000');
});