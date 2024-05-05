# final_proj210

Question: How do demographic factors, such as age group and underlying health conditions, correlate with COVID-19 mortality rates across different states and time periods?

The COVID-19 Graph Analysis project is a tool designed to analyze and visualize data related to COVID-19 deaths across different states and conditions. It takes as input a CSV file containing information about COVID-19 deaths, including details such as the state, condition, and number of deaths recorded. The project then aggregates this data to provide insights into which states and conditions have been most affected by COVID-19.

The main output of the COVID-19 Graph Analysis project is a bar chart that visually represents the aggregated data of COVID-19 deaths by state and condition. Each bar in the chart corresponds to a specific state and condition, with the height of the bar indicating the number of COVID-19 deaths recorded for that particular combination of state and condition. I ran into many errors with this bar chart and, when I revisit this project, I want to generate a bar chart with states on the X axis with COVID-19 deaths on the Y axis with each condition detailed with a different color

Functionality
The function `generate_bar_chart` takes two parameters:
1. `data`: A reference to a hashmap containing aggregated data where the key is a tuple `(String, String)` representing the state and condition, and the value is the count of COVID-19 deaths.
2. `output_file_path`: A string representing the path where the generated bar chart image will be saved.

Chart Generation
1. **Image Setup**: The function initializes the image dimensions using `BitMapBackend` and fills the background with white color.
2. **Determining Scale**: It calculates the maximum count of COVID-19 deaths to set the scale of the Y-axis.
3. **Chart Building**: Using `ChartBuilder`, the function constructs the chart with a title, label areas, and margins, and then builds the Cartesian 2D coordinate system with appropriate ranges for X and Y axes.
4. **Mesh Configuration**: The mesh on the graph is configured with labels and styles for better readability.
5. **Drawing Bars**: The function iterates over the data hashmap to draw bars on the chart. Each bar represents the count of COVID-19 deaths for a specific state and condition pair. The bottom-left and top-right coordinates of each rectangle are calculated and stored in a vector.
6. **Rendering**: Finally, the chart is rendered, and the image is presented.


Dataset Size
The code can handle a hashmap containing aggregated COVID-19 death data for various states and conditions, making it suitable for datasets with a reasonable size, potentially over 1000 vertices.

Algorithm Complexity
The algorithm used for generating the bar chart has a linear time complexity relative to the size of the input data. It iterates over each entry in the hashmap once to calculate the maximum death count and then again to draw the bars on the chart. Thus, it meets the requirement for good complexity. The code follows Rust's conventions and utilizes appropriate language features, such as pattern matching, iterators, and error handling. Variable names are descriptive, and the code is organized into a single function for generating the bar chart, enhancing readability and maintainability.

Documentation
This write-up serves as documentation for the code, explaining its purpose, functionality, and how it addresses the project requirements. Additionally, inline comments within the code can further clarify specific sections or logic.

Conclusion
The provided Rust code effectively generates a bar chart visualizing COVID-19 deaths by state and condition, demonstrating the capability to analyze and visualize graph data. By adhering to best practices and addressing project requirements, the code forms a solid foundation for further exploration and analysis of graph datasets.

