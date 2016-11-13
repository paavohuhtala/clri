# CLRi

Slow, incompatible, buggy, unsupported CLR interpreter, implemented in Rust. Requires the nightly compiler.

# What's implemented

* PE executable loader
* That's pretty much it

# Sample output

    CLRi 0.1
    Reading PE file...
    DOS magic ✓
    PE header pointer: 128
    PE magic ✓
    Section count: 3
    Section alignment: 512
    RVA count: 16
    RVA 0: DataDirectory { virtual_address: 0, size: 0 }
    RVA 1: DataDirectory { virtual_address: 8832, size: 75 }
    RVA 2: DataDirectory { virtual_address: 16384, size: 720 }
    RVA 3: DataDirectory { virtual_address: 0, size: 0 }
    RVA 4: DataDirectory { virtual_address: 0, size: 0 }
    RVA 5: DataDirectory { virtual_address: 24576, size: 12 }
    RVA 6: DataDirectory { virtual_address: 0, size: 0 }
    RVA 7: DataDirectory { virtual_address: 0, size: 0 }
    RVA 8: DataDirectory { virtual_address: 0, size: 0 }
    RVA 9: DataDirectory { virtual_address: 0, size: 0 }
    RVA 10: DataDirectory { virtual_address: 0, size: 0 }
    RVA 11: DataDirectory { virtual_address: 0, size: 0 }
    RVA 12: DataDirectory { virtual_address: 8192, size: 8 }
    RVA 13: DataDirectory { virtual_address: 0, size: 0 }
    RVA 14: DataDirectory { virtual_address: 8200, size: 72 }
    RVA 15: DataDirectory { virtual_address: 0, size: 0 }
    Section 0: SectionHeader { name: ".text", virtual_address: 8192, virtual_size: 724, real_size: 1024, offset: 512 }
    Section 1: SectionHeader { name: ".rsrc", virtual_address: 16384, virtual_size: 720, real_size: 1024, offset: 1536 }
    Section 2: SectionHeader { name: ".reloc", virtual_address: 24576, virtual_size: 12, real_size: 512, offset: 2560 }
    Managed entry point: 100663298
    Assembly has no strong name signature, skipping.
    CLR metadata header magic ✓
    CLR metadata version: 1.1
    CLR version: v4.0.30319
    Stream count: 5
    MetadataHeader { clr_version: "v4.0.30319", stream_headers: {"#US": StreamHeader { offset: 440, size: 8, name: "#US" }, "#GUID": StreamHeader { offset: 448, size: 16, name: "#GUID" }, "#Strings": StreamHeader { offset: 312, size: 128, name: "#Strings" }, "#Blob": StreamHeader { offset: 464, size: 56, name: "#Blob" }, "#~": StreamHeader { offset: 108, size: 204, name: "#~" }} }


## Useful links

[The .NET File Format](http://www.ntcore.com/files/dotnetformat.htm)

[Anatomy of a .NET assembly: PE headers](https://www.simple-talk.com/blogs/anatomy-of-a-net-assembly-pe-headers/)

[Anatomy of a .NET assembly: CLR metadata (series)](https://www.simple-talk.com/blogs/anatomy-of-a-net-assembly-clr-metadata-1/)

[ImageHlp Structures](https://msdn.microsoft.com/en-us/library/windows/desktop/ms680198(v=vs.85).aspx)

[Peering Inside the PE: A Tour of the Win32 Portable Executable File Format](https://msdn.microsoft.com/en-us/library/ms809762.aspx)

## License

Open source under the MIT license.
