#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/mol_ops.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub type RemoveHsParameters;
        pub fn new_remove_hs_parameters() -> SharedPtr<RemoveHsParameters>;

        pub fn remove_hs_parameters(
            mol: &SharedPtr<ROMol>,
            params: &SharedPtr<RemoveHsParameters>,
            sanitize: bool,
        ) -> SharedPtr<ROMol>;

        // pub fn add_hs(
        //     mol: &SharedPtr<ROMol>,
        //     explicit_only: bool,
        //     add_coords: bool,
        //     only_on_atoms: &Vec<u32>,
        //     add_residue_info: bool,
        // );

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
