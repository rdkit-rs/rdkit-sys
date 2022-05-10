#pragma once

#include "rust/cxx.h"
#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"
#include "DataStructs/ExplicitBitVect.h"
#include "GraphMol/Fingerprints/Fingerprints.h"
#include "GraphMol/MolStandardize/Tautomer.h"

namespace RDKit {
    std::shared_ptr<ROMol> copy_mol(std::shared_ptr<ROMol> mol);
    std::shared_ptr<ROMol> mol_from_smiles(const std::string &smiles);
    rust::String mol_to_smiles(std::shared_ptr<ROMol> mol);
}