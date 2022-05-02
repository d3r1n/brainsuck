
<h1 align=center>Brainsuck</h1>

<p align=center>Brainfuck but not really... like, a better version of it.</p>

<div align=center>
	<img alt="GitHub all releases" src="https://img.shields.io/github/downloads/d3r1n/brainsuck/total?color=%235caef2&label=downloads&logo=github&style=for-the-badge">
	<img alt="Lines of code" src="https://img.shields.io/tokei/lines/github/d3r1n/brainsuck?style=for-the-badge">
</div>

<h2>Install</h2>

Pre-build binaries with examples can be found in [releases](https://github.com/d3r1n/brainsuck/releases/)

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
$ brainsuck <INPUT FILE> [OPTIONS]
```

<h4>Options</h4>

- **[-m, --mem-size]:** Sets the program's memory size. 			(default: 1024)
- **[-p, --ptr-loc]:** Sets the program's memory pointer location. 	(default: 512)
- **[-a, --auto]:** Automatically allocates memory for the program. (default: false)
- **[-o, --optimize]:** Optimize the Parser and Interpreter 		(default: false)

<h2>Versions History</h2>

- **v3.0** *[Next]*
	* Adding more functionality and more keywords (Main Feature)
	* Adding a compiler based on LLVM (Main Feature)
	* Adding a transpiler to C and Rust (Main Feature)
	* Adding JiT compilation (Main Feature)
- **v2.5** *[Now]*
	* Optimized the Parser and Interpreter (Main Feature)
	* Improved speed (3x-100x times faster optimized) (Main Feature)
	* Bug fixes
- **v2.0** *[Previous]*
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
- [x]   Add optimization
- [ ]   Add custom keywords
- [ ]   Add macros
- [ ]   Add a compiler
- [ ]   Add transpiler
- [ ]   Add a JiT compiler

---

<h4>Mentions:</h4>

> [SpongeBed81](https://github.com/SpongeBed81) with his language [Yearlight](https://github.com/SpongeBed81/yearlight)
