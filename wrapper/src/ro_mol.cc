#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>
#include <DataStructs/ExplicitBitVect.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/MolStandardize/Tautomer.h>
#include <GraphMol/MolOps.h>

#include <iostream>

namespace RDKit {
    using ExplicitBitVect = ::ExplicitBitVect;

    std::shared_ptr<ROMol> copy_mol(std::shared_ptr<ROMol> mol) {
        return std::shared_ptr<ROMol>(new ROMol(*mol));
    }

    std::shared_ptr<ROMol> smiles_to_mol(const std::string &smiles) {
        ROMol *mol = SmilesToMol(smiles);

        return std::shared_ptr<ROMol>(mol);
    }

    std::shared_ptr<ROMol> smiles_to_mol_with_params(const std::string &smiles, std::shared_ptr<SmilesParserParams> params) {
        ROMol *mol = SmilesToMol(smiles, *params);

        return std::shared_ptr<ROMol>(mol);
    }
    std::shared_ptr<SmilesParserParams> new_smiles_parser_params() {
        return std::shared_ptr<SmilesParserParams>(new SmilesParserParams());
    }
    void smiles_parser_params_set_sanitize(std::shared_ptr<SmilesParserParams> params, bool sanitize) {
        params->sanitize = sanitize;
    }

    rust::String mol_to_smiles(std::shared_ptr<ROMol> mol) {
        return MolToSmiles(*mol);
    }

    std::unique_ptr<std::vector<std::string>> detect_chemistry_problems(std::shared_ptr<ROMol> mol) {
        std::vector<std::unique_ptr<RDKit::MolSanitizeException>> exceptions = RDKit::MolOps::detectChemistryProblems(*mol);
        std::vector<std::string> *get_types = new std::vector<std::string>;
        get_types->reserve(exceptions.size());

        for (auto &t: exceptions) {
            get_types->push_back(t->getType());
        }

        return std::unique_ptr<std::vector<std::string>>(get_types);
    }

    unsigned int get_num_atoms(std::shared_ptr<ROMol> mol, bool only_explicit) {
      for(auto atom : mol->atoms()) {
         auto theIdx = atom->getIdx();
         std::cout << "the idx: " << theIdx << std::endl;
      };

      return mol->getNumAtoms(only_explicit);
    }

    std::shared_ptr<Atom> get_atom_with_idx(std::shared_ptr<ROMol> mol, unsigned int idx) {
      Atom *borrowed_atom = mol->getAtomWithIdx(idx);
      Atom *atom = new Atom((*borrowed_atom));

      return std::shared_ptr<Atom>(atom);
    }

    rust::String get_symbol(std::shared_ptr<Atom> atom) {
      return atom->getSymbol();
    }

    void update_property_cache(std::shared_ptr<ROMol> mol, bool strict) {
      mol->updatePropertyCache(strict);
    }
}