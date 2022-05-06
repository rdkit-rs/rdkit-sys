0.2.4
---

 * Now using `examples/` for low level wrapper examples, making it easier to run OS X `leaks` on the wrappers
 * `RDKit::MolStandardize::TautomerEnumerator->canoncalize()` has been ported over, see the tautomer example
 * `RDKit::MolStandardize::TautomerEnumerator->enumerate()` has been ported over with `new`/`size`/`at` flavors on the Rust side