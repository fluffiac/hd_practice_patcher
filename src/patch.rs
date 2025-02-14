use crate::PatchError;
use crate::PatchError::*;

/// A section of asm that can be overwritten. Declares where
/// the code should be inserted and what it is overwriting.
/// Provides patch and unpatch methods which verify that
/// the overwritten code matches what is expected.
/// 
/// ```
/// crate::patch!(
///     BeefToFace,
///     0x02,
///     [0xbe, 0xef],
///     [0xfa, 0xce]
/// )
/// 
/// let mut buf = [0xde, 0xad, 0xbe, 0xef];
/// BeefToFace::patch(&mut buf).unwrap();
/// 
/// assert_eq!(buf, [0xde, 0xad, 0xfa, 0xce]);
/// ``` 
pub trait Patch {
    const GAME_LOC: usize;
    const GAME_ASM: &[u8];
    const PATCH_ASM: &[u8];

    fn patch(buf: &mut [u8]) -> Result<(), PatchError> {
        let end = Self::GAME_LOC + Self::GAME_ASM.len();
        let buf = buf.get_mut(Self::GAME_LOC..end).ok_or(ReadFail)?;

        if buf != Self::GAME_ASM {
            if buf == Self::PATCH_ASM {
                return Err(AlreadyPatched);
            } else {
                return Err(BinaryModified);
            }
        }

        buf.copy_from_slice(Self::PATCH_ASM);

        Ok(())
    }

    fn unpatch(buf: &mut [u8]) -> Result<(), PatchError> {
        let end = Self::GAME_LOC + Self::PATCH_ASM.len();
        let buf = buf.get_mut(Self::GAME_LOC..end).ok_or(ReadFail)?;

        if buf != Self::PATCH_ASM {
            if buf == Self::GAME_ASM {
                return Err(AlreadyUnpatched);
            } else {
                return Err(BinaryModified);
            }
        }

        buf.copy_from_slice(Self::GAME_ASM);

        Ok(())
    }
}

// helper macro for implementing patch with u8 arrays
#[macro_export]
macro_rules! patch {
    ($n:ident, $l:expr, $b:expr, $p:expr) => {
        pub struct $n;
        impl patch::Patch for $n {
            const GAME_LOC: usize = $l;
            const GAME_ASM: &[u8] = &$b;
            const PATCH_ASM: &[u8] = &$p;
        }
    };
}

// Implement patch on tuples of items that implement patch.
// In other words, if A: Patch and B: Patch, then (A, B,): Patch.
// Calling un/patch on a tuple will call the method on all of its
// members. Patch tuples can be placed inside other patch tuples.
macro_rules! patch_tuples_impl_patch {
    () => {};
    ($s:ident, $($r:ident,)*) => {
        impl<$s: Patch, $($r: Patch),*> Patch for ($s, $($r,)*) {
            // because we don't use the default implementation on
            // tuples these can be set to dummy values.
            const GAME_LOC: usize = 0;
            const GAME_ASM: &[u8] = &[];
            const PATCH_ASM: &[u8] = &[];

            fn patch(buf: &mut [u8]) -> Result<(), PatchError> {
                // call all member patch methods
                $s::patch(buf)?;
                $($r::patch(buf)?;)*
                Ok(())
            }

            fn unpatch(buf: &mut [u8]) -> Result<(), PatchError> {
                // call all member unpatch methods
                $s::unpatch(buf)?;
                $($r::unpatch(buf)?;)*
                Ok(())
            }
        }

        // recursively implement Patch for tuples of one less element.
        patch_tuples_impl_patch!($($r,)*);
    };
}

// Only implement patch for tuples of up to 12 length. This 
// restriction doesn't matter because Patch tuples can be nested
// within one another. 
patch_tuples_impl_patch!(A, B, C, D, E, F, G, H, I, J, K, L,);
