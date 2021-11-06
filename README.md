<div align="center">
	<h1>Brainsuck</h1>
	Brainfuck but not really... like... a better version of it.
</div>

<h1>Installation</h1>

Requirements:

- Rust version 1.50 or higher

<h2>Linux</h2>

```bash
curl https://raw.githubusercontent.com/d3r1n/brainsuck/master/install.sh | bash
```

<h2>Windows</h2>

- Clone the repository
- cd into `bs_bin`
- run `cargo build --release`
- generated binary will be in `target/release` (binary name: bs_bin)

<h2>Usage:</h2>

<h3>Help</h3>

```bash
$ brainsuck [-h, --help]
```

<h3>Interactive Shell</h3>

```bash
$ brainsuck
```

<h3>Execute</h3>

```bash
$ brainsuck <INPUT FILE> [OPTIONS]
```

<h4>Options</h4>

- **[-m, --mem-size]:** Sets the program's memory size. 			(default: 1024)
- **[-p, --ptr-loc]:** Sets the program's memory pointer location. 	(default: 512)
- **[-a, --auto]:** Automatically allocates memory for the program. (default: false)

---

<h2>Versions History</h2>

- **v2.0** *[Next]*
	* Adding custom keywords and functions to the brainsuck
- **v1.5** *[Now]*
	* Added **Interactive Shell** (Main Feature) `see Usage ^`
	* More bug fixes
- **v1.2** *[Previous]*
	* Automatic Memory Allocation (Main Feature)
	* New way of argument handling
	* Some bug fixes
- **v1.0**
	* Initial Verison of Brainsuck

--

<h2>Some screenshots:</h2>

<h3>Programs:</h3>

---

<h3>Hello, World!</h3>
<img src="./assets/hello_world.png" alt="">

<h3>Mandelbrot Set</h3>

<details>

<summary>Click to expand</summary>
<img src="./assets/mandel_brot.png">

</details>

<h3>Errors</h3>

---

<h3>File Not Found</h3>
<img src="./assets/file_not_found.png" alt="">

<h3>Syntax Errors</h3>
<img src="./assets/syntax_error.png" alt="">

<h3>Memory Overflow Errors</h3>
<img src="./assets/memory_overflow.png" alt="">

---

<h2>TODO:</h2>

- [X]   Add basic brainfuck commands.
- [X]   Add automatic memory allocation.
- [X]	Add repl
- [ ]   Add lots of custom commands and features.