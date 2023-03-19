#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>
#include <DataStructs/ExplicitBitVect.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/MolStandardize/Tautomer.h>

namespace RDKit {
    rust::String mol_to_inchi(std::shared_ptr<ROMol> mol);
    std::shared_ptr<ROMol> inchi_to_mol(const std::string &inchi);
}