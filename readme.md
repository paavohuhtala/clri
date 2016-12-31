# CLRi

Slow, incompatible, buggy, unsupported CLR metadata parser (hopefully an interpreter in the future), implemented in Rust. Requires the nightly compiler.

# What's implemented

* PE executable parser
* Some CLR metadata parsing
  * Image header
  * `#Strings` and `#US` string heaps
  * `#~` header
  * Some metadata table entries
  * Tagged indices
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
    RVA 1: DataDirectory { virtual_address: 8880, size: 75 }
    RVA 2: DataDirectory { virtual_address: 16384, size: 752 }
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
    Section 0: SectionHeader { name: ".text", virtual_address: 8192, virtual_size: 772, real_size: 1024, offset: 512 }
    Section 1: SectionHeader { name: ".rsrc", virtual_address: 16384, virtual_size: 752, real_size: 1024, offset: 1536 }
    Section 2: SectionHeader { name: ".reloc", virtual_address: 24576, virtual_size: 12, real_size: 512, offset: 2560 }
    Managed entry point: 100663297
    Assembly has no strong name signature, skipping.
    CLR metadata header magic ✓
    CLR metadata version: 1.1
    CLR version: v4.0.30319
    Stream count: 5
    StreamHeader { offset: 108, size: 196, name: "#~" }
    StreamHeader { offset: 304, size: 164, name: "#Strings" }
    StreamHeader { offset: 468, size: 32, name: "#US" }
    StreamHeader { offset: 500, size: 16, name: "#GUID" }
    StreamHeader { offset: 516, size: 56, name: "#Blob" }
    #~ version: 2.0
    HeapOffsetSizes { string_index: Word, guid_index: Word, blob_index: Word }
    Metadata tables: TableIds([Module, TypeRef, TypeDef, MethodDef, MemberRef, CustomAttribute, Assembly, AssemblyRef])
    Metadata tables (sorted): TableIds([InterfaceImpl, Constant, CustomAttribute, FieldMarshal, DeclSecurity, ClassLayout, FieldLayout, MethodSemantics, MethodImpl, ImplMap, FieldRVA, NestedClass, GenericParam, GenericParamConstraint])
    Metadata table row counts: {AssemblyRef: 1, TypeRef: 3, MethodDef: 1, TypeDef: 2, Module: 1, Assembly: 1, CustomAttribute: 1, MemberRef: 3}
    Metadata table index sizes: {CustomAttribute: Word, TypeRef: Word, TypeDef: Word, Module: Word, Assembly: Word, AssemblyRef: Word, MethodDef: Word, MemberRef: Word}
    Module: ModuleEntry { generation: 0, name: TableIndex(149, PhantomData), mv_id: TableIndex(1, PhantomData), enc_id: TableIndex(0, PhantomData), enc_base_id: TableIndex(0, PhantomData) }
    AsciiStringsStream { strings: ["", "<Module>", "Program", "Console", "System", "Write", "WriteLine", "Main", "Object", "HelloWorld", "RuntimeCompatibilityAttribute", "System.Runtime.CompilerServices", ".ctor", "mscorlib", "HelloWorld.exe"] }
    UserStringsStream  strings: [Valid(""), Valid("Hello, "), Valid("world!"), Valid("")] 

## Useful links

### PE

[Anatomy of a .NET assembly: PE headers](https://www.simple-talk.com/blogs/anatomy-of-a-net-assembly-pe-headers/)

[ImageHlp Structures](https://msdn.microsoft.com/en-us/library/windows/desktop/ms680198(v=vs.85).aspx)

[Peering Inside the PE: A Tour of the Win32 Portable Executable File Format](https://msdn.microsoft.com/en-us/library/ms809762.aspx)

### CLR

[CLR ECMA Specification (335)](https://www.ecma-international.org/publications/files/ECMA-ST/ECMA-335.pdf)

[The .NET File Format](http://www.ntcore.com/files/dotnetformat.htm)

[Anatomy of a .NET assembly: CLR metadata (series)](https://www.simple-talk.com/blogs/anatomy-of-a-net-assembly-clr-metadata-1/)

### Existing implementations

* [CoreCLR](https://github.com/dotnet/coreclr)
* [Mono](https://github.com/mono/mono)
* [Mono.Cecil](https://github.com/jbevain/cecil)

## License

Open source under the MIT license.
