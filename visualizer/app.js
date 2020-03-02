const width = 800;
const height = 600;

d3.csv('bounds.csv', d => ({
	x: +d.x,
	y: +d.y
})).then(data => {
	const x = d3.scaleLinear()
				.domain(d3.extent(data, d => d.x))
				.range([10, width - 10]);

	const y = d3.scaleLinear()
				.domain(d3.extent(data, d => d.y))
				.range([10, height - 10]);
	
	const line = d3.line()
				.x(d => x(d.x))
				.y(d => y(d.y));

	console.log(line(data))

	let svg = d3.select("#viz").append("svg").attr("viewBox", `0 0 ${width} ${height}`);

	svg.append("path")
		.attr("fill", "none")
		.attr("stroke", "black")
		.attr("d", line(data));
	
});
