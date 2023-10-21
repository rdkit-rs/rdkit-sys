#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");

        pub type ROMol;
        pub type ExplicitBitVect = crate::fingerprint_ffi::ExplicitBitVect;
        pub type SmilesParserParams;
        pub type Atom;

        pub type MolSanitizeException;
        pub type MolSanitizeExceptionUniquePtr; //  = UniquePtr<MolSanitizeException>;

        pub fn copy_mol(mol: SharedPtr<ROMol>) -> SharedPtr<ROMol>;

        pub fn smiles_to_mol(smi: &CxxString) -> Result<SharedPtr<ROMol>>;

        pub fn smiles_to_mol_with_params(
            smi: &CxxString,
            params: SharedPtr<SmilesParserParams>,
        ) -> Result<SharedPtr<ROMol>>;
        pub fn new_smiles_parser_params() -> SharedPtr<SmilesParserParams>;
        pub fn smiles_parser_params_set_sanitize(
            ptr: SharedPtr<SmilesParserParams>,
            sanitize: bool,
        );

        pub fn mol_to_smiles(mol: SharedPtr<ROMol>) -> String;

        pub fn detect_chemistry_problems(
            mol: SharedPtr<ROMol>,
        ) -> UniquePtr<CxxVector<MolSanitizeExceptionUniquePtr>>;

        pub fn mol_sanitize_exception_type(
            mol_sanitize_exception: &MolSanitizeExceptionUniquePtr,
        ) -> String;

        pub fn atom_sanitize_exception_get_atom_idx(
            mol_sanitize_exception: &MolSanitizeExceptionUniquePtr,
        ) -> u32;

        pub fn get_num_atoms(mol: SharedPtr<ROMol>, onlyExplicit: bool) -> u32;
        pub fn get_atom_with_idx(mol: SharedPtr<ROMol>, idx: u32) -> SharedPtr<Atom>;
        pub fn get_symbol(atom: SharedPtr<Atom>) -> String;

        pub fn update_property_cache(mol: SharedPtr<ROMol>, strict: bool);

        // RDKIT_SUBSTRUCTMATCH_EXPORT std::vector<MatchVectType> RDKit::SubstructMatch
        // ( 	const ROMol &  	mol, const ROMol &  	query,
        // const SubstructMatchParameters &  	params = SubstructMatchParameters()
        // )

        // PeriodicTable
        // const INT_VECT & 	getValenceList (UINT atomicNumber) const

        // Atom methods
        // GetIsAromatic --> isAromaticAtom
        // GetAtomicNum --> getAtomicNum
        // GetFormalCharge --> getFormalCharge
        // GetSymbol --> getSymbol
        // GetTotalNumHs --> getTotalNumHs
        // GetTotalValence --> getTotalValence
        // SetFormalCharge --> setFormalCharge
        // SetNumExplicitHs --> setNumExplicitHs
        // UpdatePropertyCache --> updatePropertyCache

        // detect_chemistry_problems needs more info on the MolSanitizeException
        // export the type to include type() and getAtomIdx()

        // Hydrogen stuff
        // export C++ struct RDKit::MolOps::RemoveHsParameters, make the members visible
        // RDKIT_GRAPHMOL_EXPORT ROMol* RDKit::MolOps::removeHs 	( 	const ROMol &  	mol,
        // 		const RemoveHsParameters &  	ps,
        // 		bool  	sanitize = true
        // 	)

        // RDKIT_GRAPHMOL_EXPORT ROMol* RDKit::MolOps::addHs 	( 	const ROMol &  	mol,
        // bool  	explicitOnly = false,
        // bool  	addCoords = false,
        // const UINT_VECT *  	onlyOnAtoms = nullptr,
        // bool  	addResidueInfo = false
        // )
    }
}
