#include "rust/cxx.h"
#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"
#include "DataStructs/ExplicitBitVect.h"
#include "GraphMol/Fingerprints/Fingerprints.h"
#include "GraphMol/MolStandardize/Tautomer.h"

namespace RDKit {
    using ExplicitBitVect = ::ExplicitBitVect;

    std::shared_ptr<ROMol> mol_from_smiles(const std::string &smiles) {
        std::shared_ptr<ROMol> mol(SmilesToMol(smiles)); // , &params);
        return mol;
    }

     rust::String mol_to_smiles(std::shared_ptr<ROMol> mol) {
        return MolToSmiles(*mol);
    }
}