# SHED: Self Hosted Executable Database
A portable key-value store which is appended to the end of an executable file. Currently only working
with ELF Executables.


## To Do
- [x] Get fully working on Linux
- [ ] Add support for Windows PE executables
- [ ] Make API look nicer
- [ ] Write some documentation

## Warning
This crate is experimental, since it involves an executable file modifying
itself it may or may not trigger antivirus products.