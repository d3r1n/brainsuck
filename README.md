
<h1 align=center>Brainsuck [WIP]</h1>

<p align=center>Brainfuck but not really... like, a better version of it.</p>

<div align=center>
	<img alt="GitHub all releases" src="https://img.shields.io/github/downloads/d3r1n/brainsuck/total?color=%235caef2&label=downloads&logo=github&style=for-the-badge">
	<img alt="Status" src="https://img.shields.io/badge/STATUS-WIP-brightgreen?style=for-the-badge">
</div>

<h2>Install</h2>

Pre-build binaries with examples can be found in [releases](https://github.com/d3r1n/brainsuck/releases/) \
To use Brainsuck's Compiler you'll need to have Rust version 1.50 or higher versions installed.

<h2>Build</h2>

Requirements:

- Rust version 1.50 or higher

<h3>Linux</h3>

```bash
curl https://raw.githubusercontent.com/d3r1n/brainsuck/master/install.sh | bash
```

<h3>Windows</h3>

- Clone the repository and cd into it
- run `cargo build --release`
- generated binary will be in `target/release` (binary name: bs_bin)

<h2>Usage:</h2>

<h4>Help</h4>

```bash
$ brainsuck [-h, --help]
```

<h4>Interactive Shell</h4>

```bash
$ brainsuck
$ brainsuck [-o, --optimize]
```

<h4>Execute</h4>

```bash
$ brainsuck [OPTIONS] <INPUT FILE>
```

<h4>Compile</h4>

```bash
$ brainsuck [-c, --compile] [OPTIONS] <INPUT FILE>
```
> Automatic allocation is not supported in compilation right now.

<h4>Options</h4>

- **[-m, --mem-size]:** Sets the program's memory size. 			(default: 1024)
- **[-p, --ptr-loc]:** Sets the program's memory pointer location. 	(default: 512)
- **[-a, --auto]:** Automatically allocates memory for the program. (default: false)
- **[-o, --optimize]:** Optimize the Parser and Interpreter 		(default: false)
- **[-c, --compile]:** Compile the program to a binary 				(default: false)

<h2>Versions History</h2>

- **v3.0** *[Next]*
	* Adding more functionality and more keywords (Main Feature)
	* Adding JiT compilation [LLVM] (Main Feature)
	* Adding LLVM compilation (Main Feature)
	* Adding support for debugging (Main Feature)
	* Adding support for making system calls (Main Feature)
	* Adding support for cross-platform compilation
- **v2.75** *[Now]*
	* Adding a compiler based on Rust (Main Feature) `see Usage ^`
	* Minor bug fixes
	* Re-structed the code
- **v2.5** *[Previous]*
	* Optimized the Parser and Interpreter (Main Feature)
	* Improved speed (3x-100x times faster optimized) (Main Feature)
	* Bug fixes
- **v2.0**
	* Added **Interactive Shell** (Main Feature) `see Usage ^`
	* More bug fixes
- **v1.5**
	* Automatic Memory Allocation (Main Feature)
	* New way of argument handling
	* Some bug fixes
- **v1.0**
	* Initial Verison of Brainsuck

<h2>Programs:</h3>

<h3>Hello, World!</h3>
<img src="./assets/hello_world.png" alt="">

<h3>Mandelbrot Set</h3>

<details>

<summary>Click to expand</summary>
<img src="./assets/mandel_brot.png">

</details>

<h2>Errors</h2>

<h3>File Not Found</h3>
<img src="./assets/file_not_found.png" alt="">

<h3>Syntax Errors</h3>
<img src="./assets/syntax_error.png" alt="">

<h3>Memory Overflow Errors</h3>
<img src="./assets/memory_overflow.png" alt="">
<img src="./assets/neg_ptr.png" alt="">

<h2>TODO:</h2>

- [X]   Add basic brainfuck commands.
- [X]   Add automatic memory allocation.
- [X]	Add repl
- [X]   Add optimization
- [X]   Add Rust based compiler
- [ ]   Add LLVM based compiler
- [ ]   Add LLVM based JiT compiler
- [ ]   Add custom keywords
- [ ]   Add support for debugging
- [ ]   Add support for making system calls
- [ ]   Add support for cross-platform compilation


---

<h4>Mentions:</h4>

> [SpongeBed81](https://github.com/SpongeBed81) with his language [Yearlight](https://github.com/SpongeBed81/yearlight)
