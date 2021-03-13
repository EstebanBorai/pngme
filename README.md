<div>
  <div align="center" style="display: block; text-align: center;">
    <img
      src="https://raw.githubusercontent.com/EstebanBorai/pngme/main/docs/png_icon.jpg"
      height="120"
      width="120"
    />
  </div>
  <h1 align="center">pngme</h1>
  <h4 align="center">Command line program that lets you hide secret messages in PNG files</h4>
</div>

## Usage

Clone `pngme` to your machine using git

```bash
git clone https://github.com/EstebanBorai/pngme.git
```

The `cd` into the `pngme` directory and run `cargo install`

```bash
cd pngme/ && cargo install
```

Then you are ready to use it by just calling `pngme`,
refer to [Commands](#commands) section for extra details on
available commands.

## Commands

### Encode a secret into a file

```bash
pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE>
```

Example:

```bash
pngme encode ./myfile.png sEcr "Hello, this is a PNG file secret"
```

### Decode a secret from a file

```bash
pngme decode <FILE_PATH> <CHUNK_TYPE>
```

Example:

```bash
pngme decode ./myfile.png sEcr
```

### Remove a secret from a file

```bash
pngme remove <FILE_PATH> <CHUNK_TYPE>
```

Example:

```bash
pngme remove ./myfile.png sEcr
```

### Print chunks

```bash
pngme print <FILE_PATH>
```

Example:

```bash
pngme print ./myfile.png
```

