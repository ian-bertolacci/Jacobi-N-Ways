include("Grid.jl")
using Grid
using ArgParse

s = ArgParseSettings()
@add_arg_table s begin
    "--N"
        help = "Edge size of computational grid"
        arg_type = Int64
        default = 1000
    "--T"
        help = "Time-steps"
        arg_type = Int64
        default = 100
end

parsed_args = parse_args(ARGS, s)

N = parsed_args["N"]
T = parsed_args["T"]

grid_r = Grid2D( N, 1.0 )

for x in grid_r.range
  for y in grid_r.range
    grid_r[x,y] = (x*y) * 1.0
  end
end

grid_w = Grid2D( grid_r )

tic()
for t in 1:T
  for x in grid_r.range
    for y in grid_r.range
      grid_w[x,y] = (grid_r[x,y] + grid_r[x-1,y] + grid_r[x,y-1] + grid_r[x+1,y] + grid_r[x,y+1])/5.0
    end
  end

  local temp = grid_r
  grid_r = grid_w
  grid_w = temp
end

time = toq()
println( "Elapsed: ", time, "s" )
