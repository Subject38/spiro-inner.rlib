use spiro_inner::*;
use std::alloc::Layout;
use std::convert::TryInto as _;
use std::ptr;

#[test]
fn test_curve() {
    let mut path: Vec<SpiroCP> = vec![
        SpiroCP {
            x: 334.,
            y: 117.,
            ty: 'v',
        },
        SpiroCP {
            x: 305.,
            y: 176.,
            ty: 'v',
        },
        SpiroCP {
            x: 212.,
            y: 142.,
            ty: 'c',
        },
        SpiroCP {
            x: 159.,
            y: 171.,
            ty: 'c',
        },
        SpiroCP {
            x: 224.,
            y: 237.,
            ty: 'c',
        },
        SpiroCP {
            x: 347.,
            y: 335.,
            ty: 'c',
        },
        SpiroCP {
            x: 202.,
            y: 467.,
            ty: 'c',
        },
        SpiroCP {
            x: 81.,
            y: 429.,
            ty: 'v',
        },
        SpiroCP {
            x: 114.,
            y: 368.,
            ty: 'v',
        },
        SpiroCP {
            x: 201.,
            y: 402.,
            ty: 'c',
        },
        SpiroCP {
            x: 276.,
            y: 369.,
            ty: 'c',
        },
        SpiroCP {
            x: 218.,
            y: 308.,
            ty: 'c',
        },
        SpiroCP {
            x: 91.,
            y: 211.,
            ty: 'c',
        },
        SpiroCP {
            x: 124.,
            y: 111.,
            ty: 'c',
        },
        SpiroCP {
            x: 229.,
            y: 82.,
            ty: 'c',
        },
    ];
    let mut segs: *mut SpiroSegment = ptr::null_mut();
    let mut i = 0;
    let mut l: Layout = Layout::new::<SpiroSegment>(); // needed to free *SpiroSegment

    const TEST_ITERATIONS: usize = 1_000;
    let path_len = path.len().try_into().unwrap();
    while i < TEST_ITERATIONS {
        unsafe {
            segs = setup_path(path.as_mut_ptr(), path_len, &mut l);
            solve_spiro(segs, path.len().try_into().unwrap());
            i += 1;
            // Don't free Spiro on last test iteration, so we can print it out.
            if i < TEST_ITERATIONS {
                free_spiro(segs, l);
            }
        }
    }

    let oplist = run_spiro(&mut path);

    println!("100 800 translate 1 -1 scale 1 setlinewidth");
    unsafe {
        spiro_to_bpath(segs, path_len, &mut bezctx_ps::POSTSCRIPT_BEZCTX);
        free_spiro(segs, l);
    }
    println!("stroke");
    println!("showpage");

    eprintln!("{:?}", oplist);
}
