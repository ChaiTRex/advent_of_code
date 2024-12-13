fn main() {
    static INPUT: &str = include_str!("../../../day13.txt");

    let mut machines = Vec::new();
    let mut iter = INPUT.lines();
    loop {
        let next_line = iter.next();
        if next_line.is_none() {
            break;
        }
        let (ax, ay) = next_line
            .unwrap()
            .split_once("Button A: X+")
            .unwrap()
            .1
            .split_once(", Y+")
            .unwrap();
        let ax = ax.parse::<i64>().unwrap();
        let ay = ay.parse::<i64>().unwrap();

        let (bx, by) = iter
            .next()
            .unwrap()
            .split_once("Button B: X+")
            .unwrap()
            .1
            .split_once(", Y+")
            .unwrap();
        let bx = bx.parse::<i64>().unwrap();
        let by = by.parse::<i64>().unwrap();

        let (px, py) = iter
            .next()
            .unwrap()
            .split_once("Prize: X=")
            .unwrap()
            .1
            .split_once(", Y=")
            .unwrap();
        let px = px.parse::<i64>().unwrap();
        let py = py.parse::<i64>().unwrap();

        let _ = iter.next();

        machines.push((ax, ay, bx, by, px, py));
    }

    static ITERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

    fn f(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> i64 {
        println!(
            "{}",
            ITERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        );
        fn extended_gcd(a: i64, b: i64) -> (i64, (i64, i64)) {
            if b == 0 {
                return (a, (1, 0));
            }
            let mut q = vec![0];
            let mut r = vec![a, b];
            let mut s = vec![1, 0];
            let mut t = vec![0, 1];

            let mut i = 2;
            loop {
                q.push(r[i - 2] / r[i - 1]);
                r.push(r[i - 2] % r[i - 1]);
                s.push(s[i - 2] - q[i - 1] * s[i - 1]);
                t.push(t[i - 2] - q[i - 1] * t[i - 1]);

                if r[i] == 0 {
                    return (r[i - 1], (s[i - 1], t[i - 1]));
                }
                i += 1;
            }
        }

        let (gx, (mut bez_ax, mut bez_bx)) = extended_gcd(ax, bx);
        let (gy, (mut bez_ay, mut bez_by)) = extended_gcd(ay, by);
        assert_eq!(ax * bez_ax + bx * bez_bx, gx);
        assert_eq!(ay * bez_ay + by * bez_by, gy);

        if px % gx != 0 || py % gy != 0 {
            return 0;
        }

        let lx = (ax * bx / gx).abs();
        let ly = (ay * by / gy).abs();
        assert_eq!(lx % ax, 0);
        assert_eq!(lx % bx, 0);
        assert_eq!(ly % ay, 0);
        assert_eq!(ly % by, 0);

        let mx = px / gx;
        let my = py / gy;
        assert_eq!(mx * gx, px);
        assert_eq!(my * gy, py);

        bez_ax *= mx;
        bez_bx *= mx;
        bez_ay *= my;
        bez_by *= my;
        assert_eq!(bez_ax * ax + bez_bx * bx, px);
        assert_eq!(bez_ay * ay + bez_by * by, py);

        let mut bez_ax_incr = lx / ax;
        let bez_bx_incr = lx / bx;
        let mut bez_ay_incr = ly / ay;
        let bez_by_incr = ly / by;
        assert_eq!(
            (bez_ax + bez_ax_incr) * ax + (bez_bx - bez_bx_incr) * bx,
            px
        );
        assert_eq!(
            (bez_ay + bez_ay_incr) * ay + (bez_by - bez_by_incr) * by,
            py
        );

        if bez_ax <= 0 {
            let n = bez_bx / bez_bx_incr;
            bez_ax += n * bez_ax_incr;
            bez_bx -= n * bez_bx_incr;
        } else {
            let n = (bez_bx.abs() + bez_bx_incr - 1) / bez_bx_incr;
            bez_ax -= n * bez_ax_incr;
            bez_bx += n * bez_bx_incr;
        }
        if bez_ax < 0 {
            return 0;
        }
        assert!(bez_ax >= 0);
        assert!(bez_bx >= 0);
        assert_eq!(bez_ax * ax + bez_bx * bx, px);

        if bez_ay <= 0 {
            let n = bez_by / bez_by_incr;
            bez_ay += n * bez_ay_incr;
            bez_by -= n * bez_by_incr;
        } else {
            let n = (bez_by.abs() + bez_by_incr - 1) / bez_by_incr;
            bez_ay -= n * bez_ay_incr;
            bez_by += n * bez_by_incr;
        }
        if bez_ay < 0 {
            return 0;
        }
        assert!(bez_ay >= 0);
        assert!(bez_by >= 0);
        assert_eq!(bez_ay * ay + bez_by * by, py);

        while bez_bx != bez_by {
            if bez_bx < bez_by {
                bez_ax -= bez_ax_incr;
                bez_bx += bez_bx_incr;
                if bez_ax < 0 {
                    return 0;
                }
            } else {
                bez_ay -= bez_ay_incr;
                bez_by += bez_by_incr;
                if bez_ay < 0 {
                    return 0;
                }
            }
        }
        assert_eq!(bez_bx, bez_by);
        assert!(bez_ax >= 0);
        assert!(bez_bx >= 0);
        assert!(bez_ay >= 0);
        assert!(bez_by >= 0);
        assert_eq!(bez_ax * ax + bez_bx * bx, px);
        assert_eq!(bez_ay * ay + bez_by * by, py);

        let mut b = bez_bx;
        let bez_b_incr = bez_bx_incr * bez_by_incr / extended_gcd(bez_bx_incr, bez_by_incr).0;
        bez_ax_incr *= bez_b_incr / bez_bx_incr;
        bez_ay_incr *= bez_b_incr / bez_by_incr;
        assert_eq!((bez_ax + bez_ax_incr) * ax + (b - bez_b_incr) * bx, px);
        assert_eq!((bez_ay + bez_ay_incr) * ay + (b - bez_b_incr) * by, py);

        while bez_ax != bez_ay {
            bez_ax -= bez_ax_incr;
            bez_ay -= bez_ay_incr;
            b += bez_b_incr;
            assert_eq!(bez_ax * ax + b * bx, px);
            assert_eq!(bez_ay * ay + b * by, py);
            if bez_ax < 0 || bez_ay < 0 {
                return 0;
            }
        }
        assert_eq!(bez_ax, bez_ay);
        assert_eq!(bez_ax * ax + b * bx, px);
        assert_eq!(bez_ay * ay + b * by, py);
        3 * bez_ax + b
    }

    let mut part1 = 0;
    for &(ax, ay, bx, by, px, py) in &machines {
        part1 += f(ax, ay, bx, by, px, py);
    }
    println!("{part1}");

    let mut part2 = 0;
    for &(ax, ay, bx, by, px, py) in &machines {
        part2 += f(ax, ay, bx, by, px + 10000000000000, py + 10000000000000);
    }
    println!("{part2}");
}
