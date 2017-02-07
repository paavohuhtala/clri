# CLRi

Slow, incompatible, buggy, unsupported CLR metadata parser (hopefully an interpreter in the future), implemented in Rust. Requires the nightly compiler.

# What's implemented

* PE executable parser
* Some CLR metadata parsing
  * Image header
  * `#Strings` and `#US` string heaps
  * `#~` header
  * Tagged/coded indices
  * The following metadata tables:
    * Module
    * TypeRef
    * TypeDef
    * MethodDef
    * MemberRef (MethodRef)
    * CustomAttribute
    * Assembly
    * AssemblyRef
* That's pretty much it

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
