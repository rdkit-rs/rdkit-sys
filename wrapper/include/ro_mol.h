#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>
#include <DataStructs/ExplicitBitVect.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/MolStandardize/Tautomer.h>

namespace RDKit {
    std::shared_ptr<ROMol> copy_mol(const std::shared_ptr<ROMol> &mol);
    std::shared_ptr<ROMol> smiles_to_mol(const std::string &smiles);
    rust::String mol_to_smiles(const std::shared_ptr<ROMol> &mol);

    std::shared_ptr<ROMol> smiles_to_mol_with_params(const std::string &smiles, const std::shared_ptr<SmilesParserParams> &params);
    std::shared_ptr<SmilesParserParams> new_smiles_parser_params();
    void smiles_parser_params_set_sanitize(const std::shared_ptr<SmilesParserParams> &params, bool sanitize);

    using MolSanitizeExceptionUniquePtr = std::unique_ptr<MolSanitizeException>;
    std::unique_ptr<std::vector<MolSanitizeExceptionUniquePtr>> detect_chemistry_problems(const std::shared_ptr<ROMol> &mol);
    rust::String mol_sanitize_exception_type(const MolSanitizeExceptionUniquePtr &mol_except);
    unsigned int atom_sanitize_exception_get_atom_idx(const MolSanitizeExceptionUniquePtr &mol_except);

    unsigned int get_num_atoms(const std::shared_ptr<ROMol> &mol, bool only_explicit);
    std::shared_ptr<Atom> get_atom_with_idx(const std::shared_ptr<ROMol> &mol, unsigned int idx);
    rust::String get_symbol(const std::shared_ptr<Atom> &atom);
    bool get_is_aromatic(const std::shared_ptr<Atom> &atom);
    int get_atomic_num(const std::shared_ptr<Atom> &atom);
    int get_formal_charge(const std::shared_ptr<Atom> &atom);
    unsigned int get_total_num_hs(const std::shared_ptr<Atom> &atom);
    unsigned int get_total_valence(const std::shared_ptr<Atom> &atom);
    void set_formal_charge(std::shared_ptr<Atom> &atom, int what);
    void set_num_explicit_hs(std::shared_ptr<Atom> &atom, int what);
    void atom_update_property_cache(std::shared_ptr<Atom> &mol, bool strict);

    void ro_mol_update_property_cache(std::shared_ptr<ROMol> &mol, bool strict);
}
