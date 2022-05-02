#include "rust/cxx.h"
#include "DataStructs/ExplicitBitVect.h"
#include "GraphMol/Fingerprints/Fingerprints.h"

namespace RDKit {
    std::shared_ptr<ExplicitBitVect> fingerprint_mol(std::shared_ptr<ROMol> mol) {
        return std::shared_ptr<ExplicitBitVect>(RDKFingerprintMol(*mol));
    }

    std::shared_ptr<ExplicitBitVect> copy_explicit_bit_vect(std::shared_ptr<ExplicitBitVect> orig) {
        std::shared_ptr<ExplicitBitVect> fingerprint(new ExplicitBitVect(*orig));
        return fingerprint;
    }

    void fingerprint_or(std::shared_ptr<ExplicitBitVect> left, std::shared_ptr<ExplicitBitVect> right) {
        left->operator|(*right);
    }

    void fingerprint_and(std::shared_ptr<ExplicitBitVect> left, std::shared_ptr<ExplicitBitVect> right) {
        left->operator&(*right);
    }

    unsigned int get_num_on_bits(std::shared_ptr<ExplicitBitVect> bitvect) {
        return bitvect->getNumOnBits();
    }
}