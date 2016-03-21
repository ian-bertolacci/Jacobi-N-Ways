module Grid
  import Base.getindex
  import Base.setindex!

  export Grid2D, Grid3D, ConstHalo

  type Grid2D
    compute_extent::Int64
    lower::Int64
    upper::Int64
    range::UnitRange{Int64}
    halo_value
    data::Array{Float64,2}

    Grid2D(N::Int64) = new( N, 1,N, 1:N, ConstHalo(0.0), zeros(Float64,N,N) )
    Grid2D(N::Int64, halo_value) = new( N, 1,N, 1:N, halo_value, zeros(Float64,N,N) )
    Grid2D(N::Int64, halo_value::Float64) = new( N, 1,N, 1:N, ConstHalo(halo_value), zeros(Float64,N,N) )
    Grid2D(other::Grid2D) = new( other.compute_extent, other.lower, other.upper, other.range, other.halo_value, copy(other.data) )
  end

  function in_extent( grid::Grid2D, x::Int64, y::Int64 )
    return x in grid.range && y in grid.range
  end

  function getindex( grid::Grid2D, x::Int64, y::Int64 )
    if in_extent(grid,x,y)
      return grid.data[x,y]
    else
      return grid.halo_value[x,y]
    end
  end

  function setindex!( grid::Grid2D, value::Float64, x::Int64, y::Int64 )
    if in_extent(grid,x,y)
      grid.data[x,y] = value
    end
    return grid[x,y];
  end

  # 3d Grid stuff

  type Grid3D
    compute_extent::Int64
    lower::Int64
    upper::Int64
    range::UnitRange{Int64}
    halo_value
    data::Array{Float64,3}

    Grid3D(N::Int64) = new( N, 1,N, 1:N, ConstHalo(0.0), zeros(Float64,N,N,N) )
    Grid3D(N::Int64, halo_value) = new( N, 1,N, 1:N, halo_value, zeros(Float64,N,N,N) )
    Grid3D(N::Int64, halo_value::Float64) = new( N, 1,N, 1:N, ConstHalo(halo_value), zeros(Float64,N,N,N) )
    Grid3D(other::Grid3D) = new( other.compute_extent, other.lower, other.upper, other.range, other.halo_value, copy(other.data) )
  end

  function in_extent( grid::Grid3D, x::Int64, y::Int64, z::Int64 )
    return x in grid.range && y in grid.range && z in grid.range
  end

  function getindex( grid::Grid3D, x::Int64, y::Int64, z::Int64 )
    if in_extent(grid,x,y,z)
      return grid.data[x,y,z]
    else
      return grid.halo_value[x,y,z]
    end
  end

  function setindex!( grid::Grid3D, value::Float64, x::Int64, y::Int64, z::Int64 )
    if in_extent(grid,x,y,z)
      grid.data[x,y,z] = value
    end
    return grid[x,y,z];
  end


  # Halo region helpers

  type ConstHalo
    value::Float64
  end

  function getindex( val::ConstHalo, args... )
    return val.value
  end
end
