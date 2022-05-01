#pragma once

#include "rust/cxx.h"
#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"
#include "DataStructs/ExplicitBitVect.h"
#include "GraphMol/Fingerprints/Fingerprints.h"
#include "GraphMol/MolStandardize/Tautomer.h"

namespace RDKit {
    std::shared_ptr<ROMol> mol_from_smiles(const std::string &smiles);
    rust::String mol_to_smiles(std::shared_ptr<ROMol> mol);

    using ExplicitBitVect = ::ExplicitBitVect;
    std::shared_ptr<ExplicitBitVect> fingerprint_mol(std::shared_ptr<ROMol> mol);
    std::shared_ptr<ExplicitBitVect> copy_explicit_bit_vect(std::shared_ptr<ExplicitBitVect> orig);

    void fingerprint_or(std::shared_ptr<ExplicitBitVect> left, std::shared_ptr<ExplicitBitVect> right);
    void fingerprint_or_alt(std::shared_ptr<ExplicitBitVect> left, std::shared_ptr<ExplicitBitVect> right);
    void fingerprint_and(std::shared_ptr<ExplicitBitVect> left, std::shared_ptr<ExplicitBitVect> right);
    unsigned int get_num_on_bits(std::shared_ptr<ExplicitBitVect> bitvect);

    using TautomerEnumerator = RDKit::MolStandardize::TautomerEnumerator;
    using TautomerEnumeratorResult = RDKit::MolStandardize::TautomerEnumeratorResult;
    std::shared_ptr<TautomerEnumerator> tautomer_enumerator();
    std::shared_ptr<TautomerEnumeratorResult> tautomer_enumerate(std::shared_ptr<TautomerEnumerator> enumerator, std::shared_ptr<ROMol> mol);
}