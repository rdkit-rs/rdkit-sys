#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <RDGeneral/FileParseException.h>
#include <GraphMol/FileParsers/FileParsers.h>

namespace RDKit {
    std::shared_ptr<RWMol> rw_mol_from_mol_block(const std::string &mol_block, bool sanitize, bool remove_hs, bool strict_parsing) {
        RWMol *mol;
        try {
            mol = MolBlockToMol(mol_block, sanitize, remove_hs, strict_parsing);
        } catch (const RDKit::FileParseException &e) {
            mol = nullptr;
        } catch (const RDKit::AtomValenceException &e) {
            mol = nullptr;
        }
        return std::shared_ptr<RWMol>(mol);
    }

    std::shared_ptr<RWMol> rw_mol_from_ro_mol(std::shared_ptr<ROMol> mol, bool quick_copy, int conf_id) {
        RWMol *rw_mol = new RWMol(*mol, quick_copy, conf_id);
        return std::shared_ptr<RWMol>(rw_mol);
    }

    std::shared_ptr<RWMol> rw_mol_from_rw_mol(std::shared_ptr<RWMol> mol) {
        RWMol *rw_mol = new RWMol(*mol);
        return std::shared_ptr<RWMol>(rw_mol);
    }
}