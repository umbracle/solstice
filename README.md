# Solstice

Solstice is a VSCode extension for Solidity that adds debugging and execution tracing capabilities. It lets you trace and debug the execution of Solidity tests directly in your editor. The extension provides real-time visualization of contract state, memory, storage, and execution flow, making it easier to understand and troubleshoot smart contract behavior.

The language support in Solstice is forked from the [Solang VSCode extension](https://github.com/hyperledger-solang/solang/tree/main/src/bin/languageserver). This fork maintains all the language features from Solang while adding specialized debugging tools. Long-term, the plan is to migrate to the [Solar](https://github.com/paradigmxyz/solar) parser library when it's production ready.

## Usage

Build the Rust server:

```
$ cargo build
```

Build the VSCode extension:

```
$ npm run compile
```

Open `Solstice` in Vscode and Open the `Run and Debug` tab. Execute the `Launch Client` configuration and open any Solidity project.
