const { createHash } = require('crypto');

process.once('loaded', () => {
    global.createHash = createHash;
});
