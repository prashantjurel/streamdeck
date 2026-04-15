const fs = require('fs');
const path = require('path');

const assetsDir = path.join(__dirname, 'assets');
const pngPath = path.join(assetsDir, 'icon_256.png');
const icoPath = path.join(assetsDir, 'icon.ico');

try {
    if (!fs.existsSync(pngPath)) {
        console.error('PNG not found at ' + pngPath);
        process.exit(1);
    }

    const pngData = fs.readFileSync(pngPath);
    const pngSize = pngData.length;

    const header = Buffer.alloc(6);
    header.writeUInt16LE(0, 0); 
    header.writeUInt16LE(1, 2); 
    header.writeUInt16LE(1, 4); 

    const entry = Buffer.alloc(16);
    entry.writeUInt8(0, 0); 
    entry.writeUInt8(0, 1); 
    entry.writeUInt8(0, 2); 
    entry.writeUInt8(0, 3); 
    entry.writeUInt16LE(1, 4); 
    entry.writeUInt16LE(32, 6); 
    entry.writeUInt32LE(pngSize, 8); 
    entry.writeUInt32LE(22, 12); 

    const icoData = Buffer.concat([header, entry, pngData]);
    fs.writeFileSync(icoPath, icoData);
    console.log('Successfully created ico at ' + icoPath);
} catch (err) {
    console.error('Failed to create ICO:', err);
    process.exit(1);
}
