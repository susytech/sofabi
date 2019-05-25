# sofabi

[![Build Status][travis-image]][travis-url][![Build coverage][coveralls-image]][coveralls-url]

[travis-image]: https://travis-ci.org/susytech/sofabi.svg?branch=master
[travis-url]: https://travis-ci.org/susytech/sofabi
[coveralls-image]: https://coveralls.io/repos/github/susytech/sofabi/badge.svg?branch=master
[coveralls-url]: http://coveralls.io/github/susytech/sofabi?branch=master

The ABI, Application Binary Interface, is basically how you call functions in a contract and get data back.

> An ABI determines such details as how functions are called and in which binary format information should be passed from one program component to the next...

An Sophon smart contract is bytecode, SVM, on the Sophon blockchain. Among the SVM, there could be several functions in a contract. An ABI is necessary so that you can specify which function in the contract to invoke, as well as get a guarantee that the function will return data in the format you are expecting. [read more](http://sophon.stackexchange.com/a/1171/394)

This library encodes function calls and decodes their output.

[Documentation](https://docs.rs/sofabi)

### Installation

- via cargo

  ```
  cargo install sofabi-cli
  ```

- via homebrew

  ```
  brew tap susytech/susytech
  brew install sofabi
  ```

### Usage

```
Sophon ABI coder.
  Copyleft 2016-2017 Susy Technologies (UK) Limited

Usage:
    sofabi encode function <abi-path> <function-name> [-p <param>]... [-l | --lenient]
    sofabi encode params [-v <type> <param>]... [-l | --lenient]
    sofabi decode function <abi-path> <function-name> <data>
    sofabi decode params [-t <type>]... <data>
    sofabi decode log <abi-path> <event-name> [-l <topic>]... <data>
    sofabi -h | --help

Options:
    -h, --help         Display this message and exit.
    -l, --lenient      Allow short representation of input params.

Commands:
    encode             Encode ABI call.
    decode             Decode ABI call result.
    function           Load function from json ABI file.
    params             Specify types of input params inline.
    log                Decode event log.
```

### Examples

```
sofabi encode params -v bool 1
```

> 0000000000000000000000000000000000000000000000000000000000000001

--

```
sofabi encode params -v bool 1 -v string gavofyork -v bool 0
```

> 00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000096761766f66796f726b0000000000000000000000000000000000000000000000

--

```
sofabi encode params -v bool[] [1,0,false]
```

> 00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000

--

```
sofabi encode function examples/test.json foo -p 1
```

```json
[{
	"type":"function",
	"inputs": [{
		"name":"a",
		"type":"bool"
	}],
	"name":"foo",
	"outputs": []
}]
```

> 455575780000000000000000000000000000000000000000000000000000000000000001

--

```
sofabi decode params -t bool 0000000000000000000000000000000000000000000000000000000000000001
```

> bool true

--

```
sofabi decode params -t bool -t string -t bool 00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000096761766f66796f726b0000000000000000000000000000000000000000000000
```

> bool true<br/>
> string gavofyork<br/>
> bool false

--

```
sofabi decode params -t bool[] 00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```

> bool[] [true,false,false]

--

```
sofabi decode function ./examples/foo.json bar 0000000000000000000000000000000000000000000000000000000000000001
```

```json
[{
	"constant":false,
	"inputs":[{
		"name":"hello",
		"type":"address"
	}],
	"name":"bar",
	"outputs":[{
		"name":"",
		"type":"bool"
	}],
	"type":"function"
}]
```

> bool true

--

```
sofabi decode log ./examples/event.json Event -l 0000000000000000000000000000000000000000000000000000000000000001 0000000000000000000000004444444444444444444444444444444444444444
```

> a bool true<br/>
> b address 4444444444444444444444444444444444444444

### Alternative tools

-  [sofabi-js](https://github.com/jacogr/sofabi-js) - javascript port of this library created by [@jacogr](https://github.com/jacogr)

## Susy Sophon toolchain

In addition to the Susy Sophon client, there are additional tools in this repository available:

- [svmbin](https://github.com/susytech/susy-sophon/blob/master/svmbin/) - SVM implementation for Susy Sophon.
- [sofabi](https://github.com/susytech/sofabi) - Susy Sophon function calls encoding.
- [sofstore](https://github.com/susytech/susy-sophon/blob/master/sofstore/) - Susy Sophon key management.
- [sofkey](https://github.com/susytech/susy-sophon/blob/master/sofkey/) - Susy Sophon keys generator.
- [whisper](https://github.com/susytech/susy-sophon/blob/master/whisper/) - Implementation of Whisper-v2 PoC.