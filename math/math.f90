module math 
    use class_parser
    use ast
    use symtab
    use ansi_colors
    use, intrinsic :: iso_fortran_env, only : dw => real64
    implicit none
    private
    public :: factorial , sgn
contains
    function sgn(n) result(res)
        real(kind=dw), intent(in) :: n
        real(kind=dw) :: res

        if (n > 0) then
            res = 1
        else if (n < 0) then 
            res = -1
        else 
            res = 0
        end if
    end function sgn

    recursive function factorial(n) result(res)
        real(kind=dw), intent(in) :: n
        real(kind=dw) :: res
        
        if (n <= 1.0_dw) then 
            res = 1.0_dw
        else 
            res = n*factorial(n - 1.0_dw) 
        end if
    end function factorial
end module math 