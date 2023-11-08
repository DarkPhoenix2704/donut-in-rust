fn main() {
    print!("\x1b[H");
    const SCREEN_H: usize = 32;
    const SCREEN_W: usize = 32;

    let r1 = 1.0;
    let r2 = 2.0;

    let k2 = 7.0;
    let k1 = SCREEN_H as f64 * k2  * 3. /( 8. * (r1+r2));

    let mut a = 0.0;
    let mut b = 0.0;

    loop {
        print!("\x1b[H");

        let mut output = [[' '; SCREEN_W]; SCREEN_H];
        let mut zbuffer = [[0.0; SCREEN_W]; SCREEN_H];

        let mut theta = 0.;

        let (sin_a, cos_a) = f64::sin_cos(a);
        let (sin_b, cos_b) = f64::sin_cos(b);

        while theta < 6.28 {
            let (sin_theta, cos_theta) = f64::sin_cos(theta);
            let mut phi = 0.0; 

            while phi < 6.28 {
                    let (sin_phi, cos_phi) = f64::sin_cos(phi);

                    let cir_x = r2 + (r1 * cos_theta);
                    let cir_y = r1 * sin_theta;

                    let x = cir_x * (cos_b*cos_phi + sin_a*sin_b*sin_phi) - cir_y*cos_a*sin_b;
                    let y = cir_x * (sin_b*cos_phi - sin_a*cos_b*sin_phi) + cir_y*cos_a*cos_b;
                    let z = k2 + cos_a * cir_x * sin_phi + cir_y*sin_a;

                    let ooz = 1.0/z;

                    let x_pro = ((SCREEN_W as f64)/2.0 + k1*ooz*x) as usize;
                    let y_pro = ((SCREEN_H as f64)/2.0 + k1*ooz*y) as usize;

                    let lum = cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a*sin_theta + cos_b*(cos_a*sin_theta - cos_theta*sin_a*sin_phi);

                    if lum > 0.0 {
                        if ooz > zbuffer[x_pro][y_pro] {
                            zbuffer[x_pro][y_pro] = ooz;

                            let lum_index: usize = (lum * 8.0) as usize;

                            output[x_pro][y_pro] = b".,-~:;=!*#$@"[lum_index] as char
                        }
                    }
                    phi += 0.02;
                }
                theta +=0.02;
        }

         print!("\x1b[H");
        for i in 0..SCREEN_H as usize {
            for j in 0..SCREEN_W as usize {
                print!("{}", output[i][j])
            }  
            print!("\n")          
        }
        a += 0.012;
        b += 0.005;

        std::thread::sleep(std::time::Duration::new(0, 5000000))
    }
}