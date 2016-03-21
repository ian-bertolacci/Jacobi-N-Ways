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

println( "N: $N\nT: $T" )

grid_r = Grid3D( N, 1.0 )

for x in grid_r.range
  for y in grid_r.range
    for z in grid_r.range
      grid_r[x,y,z] = ((.1+x)*(.2+y)*(.3+z)) * 1.0
    end
  end
end

grid_w = Grid3D( grid_r )

tic()
for t in 1:T
  for x in grid_r.range
    for y in grid_r.range
      for z in grid_r.range
        grid_w[x,y,z] = (grid_r[x,y,z] + grid_r[x-1,y,z] + grid_r[x,y-1,z] + grid_r[x,y,z-1] +
                                         grid_r[x+1,y,z] + grid_r[x,y+1,z] + grid_r[x,y,z+1] ) / 7.0
      end
    end
  end

  local temp = grid_r
  grid_r = grid_w
  grid_w = temp
end

time = toq()
println( "Elapsed: ", time, "s" )
