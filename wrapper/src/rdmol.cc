#include "rust/cxx.h"
#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"
#include "DataStructs/ExplicitBitVect.h"
#include "GraphMol/Fingerprints/Fingerprints.h"
#include "GraphMol/MolStandardize/Tautomer.h"

namespace RDKit {
    using ExplicitBitVect = ::ExplicitBitVect;
    using TautomerEnumerator = MolStandardize::TautomerEnumerator;
    using TautomerEnumeratorResult = MolStandardize::TautomerEnumeratorResult;

    std::shared_ptr<ROMol> mol_from_smiles(const std::string &smiles) {
        std::shared_ptr<ROMol> mol(SmilesToMol(smiles)); // , &params);
        return mol;
    }

     rust::String mol_to_smiles(std::shared_ptr<ROMol> mol) {
        return MolToSmiles(*mol);
    }

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

    std::shared_ptr<TautomerEnumerator> tautomer_enumerator() {
        TautomerEnumerator *enumerator = new MolStandardize::TautomerEnumerator(new MolStandardize::TautomerCatalog());
        return std::shared_ptr<TautomerEnumerator>(enumerator);
    }

    std::shared_ptr<TautomerEnumeratorResult> tautomer_enumerate(std::shared_ptr<TautomerEnumerator> enumerator, std::shared_ptr<ROMol> mol) {
        TautomerEnumeratorResult stacked_enumerator = enumerator->enumerate(*mol);
        TautomerEnumeratorResult *heaped_enumerator = new TautomerEnumeratorResult(stacked_enumerator);
        return std::shared_ptr<TautomerEnumeratorResult>(heaped_enumerator);
    }
}