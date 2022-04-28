#pragma once

#include "rust/cxx.h"
#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"
#include "DataStructs/ExplicitBitVect.h"
#include "GraphMol/Fingerprints/Fingerprints.h"

namespace RDKit {
    std::shared_ptr<ROMol> mol_from_smiles(const std::string &smiles);
    rust::String mol_to_smiles(std::shared_ptr<ROMol> mol);

    using ExplicitBitVect = ::ExplicitBitVect;
    ExplicitBitVect *fingerprint_mol(std::shared_ptr<ROMol> mol);
    ExplicitBitVect *copy_explicit_bit_vect(ExplicitBitVect *orig);
    void fingerprint_or(ExplicitBitVect* left, ExplicitBitVect* right);
    void fingerprint_and(ExplicitBitVect* left, ExplicitBitVect* right);
    unsigned int get_num_on_bits(ExplicitBitVect *bitvect);
}