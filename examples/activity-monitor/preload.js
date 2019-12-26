const { cpus } = require('os');

process.once('loaded', () => {
    global.cpus = cpus;
});
