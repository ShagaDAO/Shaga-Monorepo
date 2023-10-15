const path = require('path');
const { spawn } = require('child_process');
const { Solita } = require('@metaplex-foundation/solita');
const { writeFile } = require('fs/promises');

const PROGRAM_NAME = 'shaga';
const PROGRAM_ID = '9SwYZxTQUYruFSHYeTqrtB5pTtuGJEGksh7ufpNS1YK5';

const programDir = path.join(__dirname, '..', 'programs/shagajoe');
const generatedIdlDir = path.join(__dirname, 'idl');
const generatedSDKDir = path.join(__dirname, 'src', 'generated');

async function main() {
  const anchor = spawn("anchor", ['build', '--idl', generatedIdlDir], { cwd: programDir })
    .on('error', (err) => {
      console.error(err);
      // @ts-ignore this err does have a code
      if (err.code === 'ENOENT') {
        console.error(
          'Ensure that `anchor` is installed and in your path. anchor 0.28.0 is only available in github for now as of Jul 2023.',
        );
      }
      process.exit(1);
    })
    .on('exit', () => {
      console.log('IDL written to: %s', path.join(generatedIdlDir, `${PROGRAM_NAME}.json`));
      generateTypeScriptSDK();
    });

  anchor.stdout.on('data', (buf) => console.log(buf.toString('utf8')));
  anchor.stderr.on('data', (buf) => console.error(buf.toString('utf8')));
}

async function generateTypeScriptSDK() {
  console.error('Generating TypeScript SDK to %s', generatedSDKDir);
  const generatedIdlPath = path.join(generatedIdlDir, `${PROGRAM_NAME}.json`);

  const idl = require(generatedIdlPath);
  if (idl.metadata?.address == null) {
    idl.metadata = { ...idl.metadata, address: PROGRAM_ID };
    await writeFile(generatedIdlPath, JSON.stringify(idl, null, 2));
  }
  const gen = new Solita(idl, { formatCode: true });
  await gen.renderAndWriteTo(generatedSDKDir);

  console.error('Success!');

  process.exit(0);
}

// main().catch((err) => {
//   console.error(err)
//   process.exit(1)
// })
generateTypeScriptSDK().catch((err) => {
  console.error(err)
  process.exit(1)
})