#pragma once

#include "rust/cxx.h"

namespace RDKit {
    using ExplicitBitVect = ::ExplicitBitVect;
    std::shared_ptr<ExplicitBitVect> fingerprint_mol(std::shared_ptr<ROMol> mol);
    std::shared_ptr<ExplicitBitVect> copy_explicit_bit_vect(std::shared_ptr<ExplicitBitVect> orig);
    void fingerprint_or(std::shared_ptr<ExplicitBitVect> left, std::shared_ptr<ExplicitBitVect> right);
    void fingerprint_or_alt(std::shared_ptr<ExplicitBitVect> left, std::shared_ptr<ExplicitBitVect> right);
    void fingerprint_and(std::shared_ptr<ExplicitBitVect> left, std::shared_ptr<ExplicitBitVect> right);
    unsigned int get_num_on_bits(std::shared_ptr<ExplicitBitVect> bitvect);

    std::unique_ptr<std::vector<uint8_t>> explicit_bit_vect_to_bytes_vec(std::shared_ptr<ExplicitBitVect> bitvect);
}