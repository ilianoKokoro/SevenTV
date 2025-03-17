mod worker;

fn main() -> Result<(), anyhow::Error> {
	let args: Vec<String> = std::env::args().collect();

	let data = std::fs::read(&args[1])?;
	let mut decoder = FfmpegDecoder::new(&Task { ..Default::default() }, Cow::Borrowed(&data))?;

	let task_info = Task {
		output: Output {
			formats: vec![
				OutputFormatOptions {
					format: OutputFormat::WebpAnim as i32,
					quality: 90,
					..Default::default()
				},
				OutputFormatOptions {
					format: OutputFormat::PngStatic as i32,
					quality: 100,
					..Default::default()
				},
			],
			resize: Resize::Heights(IntegerList { values: vec![64, 96] }).into(),
			upscale: true,
			disable_two_pass_decoding: true,
			disable_resize_chaining: true,

			..Default::default()
		}
		.into(),
		..Default::default()
	};

	let mut task = worker::process::blocking::BlockingTask::new(&task_info, &data)?;

	dbg!(decoder.info().timescale);
	while let Some(frame) = decoder.decode()? {
		dbg!(frame.duration_ts);
	}
	while task.drive().unwrap_or(false) {}
	let output = task.finish()?;

	let encoded = &output
		.output
		.iter()
		.find(|i| i.format == OutputFormat::WebpAnim)
		.unwrap()
		.data;

	std::fs::write("out.webp", encoded)?;

	let mut decoder = WebpDecoder::new(&Task { ..Default::default() }, Cow::Borrowed(encoded))?;
	dbg!(decoder.info().timescale);
	while let Some(frame) = decoder.decode()? {
		dbg!(frame.duration_ts);
	}

	Ok(())
}
