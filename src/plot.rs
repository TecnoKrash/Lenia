use plotters::prelude::*;

use crate::growth::*;
use crate::imgep::*;


pub fn plot_test() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("storage/plots/test.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x²", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, gaussian(0.5,0.15, x.into()) as f32)),
            &RED,
        ))?;

    /*
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
        */

    root.present()?;

    Ok(())
}

fn aux_kernels(i: usize, p: &Param, x: f32) -> f32{
    let mut res = 0.0;
    for j in 0..p.nb_bump[i]{
            res += (p.b[i][j]*gaussian(p.a[i][j], p.w[i][j], (x.abs()).into())) as f32;
    }
    
    res
}


pub fn plot_kernels(p: &Param, k_sum: &Vec<f64>, name: &str, caption: &str, colors: &Vec<(u8, u8, u8)>) -> Result<(), Box<dyn std::error::Error>>{
    
    let mut func: Vec<Box<dyn Fn(f32) -> f32>> = Vec::with_capacity(p.nb_kernels);

    for i in 0..p.nb_kernels {
        func.push(Box::new( move |x: f32| { aux_kernels(i, &p, x)/(k_sum[i] as f32) }));
    }

    plot_functions(name, caption, &func, &colors)?;

    Ok(())
}


pub fn plot_growth(p: &Param, name: &str, caption: &str, colors: &Vec<(u8, u8, u8)>) -> Result<(), Box<dyn std::error::Error>>{
    
    let mut func: Vec<Box<dyn Fn(f32) -> f32>> = Vec::with_capacity(p.nb_kernels);

    for i in 0..p.nb_kernels {
        func.push(Box::new( move |x: f32| { (p.h[i]*(-1.0 + 2.0*gaussian(p.mu[i], p.sigma[i], x.into()))) as f32 }));
    }

    plot_functions(name, caption, &func, &colors)?;

    Ok(())
}


pub fn plot_bruit(seed: &Seed, ampli: f64, name: &str, caption: &str, colors: &Vec<(u8, u8, u8)>) -> Result<(), Box<dyn std::error::Error>>{
    
    let mut func: Vec<Box<dyn Fn(f32) -> f32>> = vec![];

    func.push(Box::new( move |t: f32| { bruit_rand(&seed, 0, 0, t.into(), ampli) as f32}));

    plot_functions(name, caption, &func, &colors)?;

    Ok(())
}


pub fn plot_functions<F: Sized>(name: &str, caption: &str, func: &Vec<F>, colors: &Vec<(u8,u8,u8)>) -> Result<(), Box<dyn std::error::Error>> where F: Fn(f32) -> f32, {

    let path = format!("storage/plots/{}.png",name);

    let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..10f32, -1.5f32..3.5f32)?;

    chart.configure_mesh().draw()?;

    for i in 0..func.len(){
        chart
            .draw_series(LineSeries::new(
                    (-1000..=1000).map(|x| (x as f32 / 100.0)*10.0).map(|x| (x, func[i](x))),
                    &RGBColor(colors[i].0,colors[i].1,colors[i].2),
                    ))?;
    }

    root.present()?;

    Ok(())
}
