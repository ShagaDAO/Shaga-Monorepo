const path = require('path');
const programDir = path.join(__dirname, '..', 'programs/shagajoe');
const idlDir = path.join(__dirname, 'idl');
const sdkDir = path.join(__dirname, 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'shaga_joe',
  programId: '9SwYZxTQUYruFSHYeTqrtB5pTtuGJEGksh7ufpNS1YK5',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
