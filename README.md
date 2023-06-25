# TravellingSalesmanProblem
This project implements 5 algorithms to solve the travelling salesman problem.
1. **Brute force**: does what it says, tries to be a bit efficient
2. **Radial Search**: finds the average of all the points and spins a line around it, it will add points in the order that the line encounters them
3. **Closest neighbour**: it picks a random point, and then hops to the next closes point. Etc.
4. **Random**: orders them randomly (for comparison purposes)
5. **Triangle algorithm**: Starts with 3 random points (a triangle), and tries to expand sides of the triangle

# Results
100 problems needed to be solved; 9 cities in each problem
* Brute force:       tot_dis: 5516.819720261234, tot_perfect: 100
* Radial Search:     tot_dis: 5696.949323557044, tot_perfect: 32
* Nearest neighbour: tot_dis: 6051.998809986027, tot_perfect: 15
* Random:            tot_dis: 9295.063939802758, tot_perfect: 0
* Triangle expansion:tot_dis: 6001.752322262696, tot_perfect: 11
