#include "rust/cxx.h"
#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"
#include "DataStructs/ExplicitBitVect.h"
#include "GraphMol/Fingerprints/Fingerprints.h"

namespace RDKit {
    using ExplicitBitVect = ::ExplicitBitVect;

    std::shared_ptr<ROMol> mol_from_smiles(const std::string &smiles) {
        std::shared_ptr<ROMol> mol(SmilesToMol(smiles)); // , &params);
        return mol;
    }

     rust::String mol_to_smiles(std::shared_ptr<ROMol> mol) {
        return MolToSmiles(*mol);
    }

    ExplicitBitVect *fingerprint_mol(std::shared_ptr<ROMol> mol) {
        return RDKFingerprintMol(*mol);
    }

    ExplicitBitVect *copy_explicit_bit_vect(ExplicitBitVect *orig) {
        return new ExplicitBitVect(*orig);
    }

    void fingerprint_or(ExplicitBitVect* left, ExplicitBitVect* right) {
        left->operator|(*right);
    }

    void fingerprint_and(ExplicitBitVect* left, ExplicitBitVect* right) {
        left->operator&(*right);
    }

    unsigned int get_num_on_bits(ExplicitBitVect *bitvect) {
        return bitvect->getNumOnBits();
    }
}