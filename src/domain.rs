pub use algebra::AffineCurve;
pub use algebra::Field;

use mina_curves::pasta::pallas as Pallas;
pub use Pallas::Affine as PallasPoint;
pub type PallasField = <PallasPoint as AffineCurve>::BaseField;
pub type PallasScalar = <PallasPoint as AffineCurve>::ScalarField;