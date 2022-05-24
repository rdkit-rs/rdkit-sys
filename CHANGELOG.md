0.2.10
---

 * update bridge to catch atomvalenceexception and return nullptr

0.2.9
---

 * convert fingerprint to vec, switching from boost dynamic_bitvect to rust bitvec for ergonomics

0.2.8
---

 * RWMol molblock support

0.2.7
---

 * build.rs refactoring: introspect bridge files to build list of .cc/.h files

0.2.6
---

 * Substruct match

0.2.5
---

 * ROMol copy

0.2.4
---

 * Now using `examples/` for low level wrapper examples, making it easier to run OS X `leaks` on the wrappers
 * `RDKit::MolStandardize::TautomerEnumerator->canoncalize()` has been ported over, see the tautomer example
 * `RDKit::MolStandardize::TautomerEnumerator->enumerate()` has been ported over with `new`/`size`/`at` flavors on the Rust side