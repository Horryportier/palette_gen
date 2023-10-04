# Palette_gen
> An simpple tool for generating color pallets from images with configs for some tools. 

This tool generates color palette of 8 (by default) colors in hex format. 
To see avalible implementations see [impls.md](https://github.com/Horryportier/palette_gen/blob/main/impls/impls.md)

# Ho to use it 
Give it path to image and optional safe path if safe path is not provided it will try to read [PALETTE_SAVE_FILE] env variable.
To see mor options use `--help | -h`
``` 
palette_gen -i /path/to/img -s /path/to/safe/file
```

# TODO'S
- Support for more Safe formats 'json,xml, ect.'
- more plugins for more tools (feel free to impl one yourself)
- better system for creating readable color pallets (monochromatic dosen't work well)
- automaticly change wallpaper at the same time 
