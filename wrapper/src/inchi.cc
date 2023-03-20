#include "rust/cxx.h"
#include <GraphMol/inchi.h>

namespace RDKit {
    rust::String mol_to_inchi(std::shared_ptr<ROMol> mol) {
        ExtraInchiReturnValues rv;
        return MolToInchi(*mol, rv, "");
    }

    std::shared_ptr<ROMol> inchi_to_mol(const std::string &inchi) {
        ExtraInchiReturnValues rv;
        ROMol *mol = InchiToMol(inchi, rv);
        return std::shared_ptr<ROMol>(mol);
    }

    rust::String inchi_to_inchi_key(const std::string &inchi) {
        return InchiToInchiKey(inchi);
    }
}
