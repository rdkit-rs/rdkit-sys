#include "GraphMol/GraphMol.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
//#include "GraphMol/SmilesParse/SmilesWrite.h"

namespace RDKit {
    std::shared_ptr<ROMol> mol_from_smiles(const std::string &smiles) {
        std::shared_ptr<ROMol> mol(SmilesToMol(smiles)); // , &params);
        return mol;
    }
}