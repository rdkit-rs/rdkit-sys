#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>
#include <DataStructs/ExplicitBitVect.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/MolStandardize/Tautomer.h>
#include <GraphMol/MolOps.h>
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
}