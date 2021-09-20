// Copyright (c) 2021, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

pub trait Vec2<T>
{
    fn x(&mut self) -> &mut T;
    fn y(&mut self) -> &mut T;
}

pub struct Vector<T: Sized, const N: usize>
{
    data: [T; N]
}

impl<T: Sized, const N: usize> Vector<T, N>
{
    pub fn new(data: [T; N]) -> Vector<T, N>
    {
        return Vector {
            data
        };
    }
}

impl<T: Sized, T1: Into<T>> From<(T1, T1)> for Vector<T, 2>
{
    fn from((x, y): (T1, T1)) -> Vector<T, 2>
    {
        return Vector::new([x.into(), y.into()]);
    }
}

impl<T: Sized> Vec2<T> for Vector<T, 2>
{
    fn x(&mut self) -> &mut T
    {
        return &mut self.data[0];
    }

    fn y(&mut self) -> &mut T
    {
        return &mut self.data[1];
    }
}

impl<T: Sized + Clone, const N: usize> Clone for Vector<T, N>
{
    fn clone(&self) -> Vector<T, N>
    {
        return Vector::new(self.data.clone());
    }
}

impl<T: Sized + Clone + Copy, const N: usize> Copy for Vector<T, N> {}

pub type Vec2f = Vector<f32, 2>;
