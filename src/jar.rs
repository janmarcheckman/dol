#[derive(Debug, Clone)]
pub struct Jar<T>
{
    pub values: Vec<T>,
    pub begin_indices: Vec<usize>,
}

impl<T> Jar<T> where T: Clone
{
    pub fn new() -> Jar<T>
    {
        Jar
        {
            values: Vec::new(),
            begin_indices: Vec::new(),
        }
    }

    pub fn len(&self) -> usize
    {
        self.begin_indices.len()
    }
    
    fn get_begin_end(&self, index: usize) -> (usize, usize)
    {
        let begin = self.begin_indices[index];
        let next = index + 1;
        let end = if next < self.begin_indices.len() { self.begin_indices[next] } else { self.values.len() };
        (begin, end)
    }

    pub fn get_range(&self, index: usize) -> &[T]
    {
        let (begin, end) = self.get_begin_end(index);
        &self.values[begin..end]
    }

    pub fn get_range_mut(&mut self, index: usize) -> &mut [T]
    {
        let (begin, end) = self.get_begin_end(index);
        &mut self.values[begin..end]
    }

    pub fn append_empty_range(&mut self)
    {
        self.begin_indices.push(self.values.len());
    }
    
    pub fn append_range(&mut self, ts: &[T])
    {
        self.append_empty_range();
        for t in ts
        {
            self.values.push(t.clone())
        }
    }
    
    pub fn append_to_range(&mut self, index: usize, t: T)
    {
        self.values.insert(self.begin_indices[index], t);
        for i in index+1..self.begin_indices.len()
        {
            self.begin_indices[i] += 1;
        }
    }
    
    pub fn append_to_last_range(&mut self, t: T)
    {
        self.values.push(t);
    }
      
    pub fn append_slice_to_range(&mut self, index: usize, ts: &[T])
    {
        for t in ts
        {
            self.append_to_range(index, t.clone())
        }
    }
    
    pub fn remove(&mut self, index: usize)
    {
        let (begin, end) = self.get_begin_end(index);
        let l = end - begin;
        for _ in 0..l
        {
            self.values.remove(begin);
        }
        self.begin_indices.remove(index);
        for i in index..self.begin_indices.len()
        {
            self.begin_indices[i] -= l;
        }
    }
    
    pub fn clear(&mut self)
    {
        self.values.clear();
        self.begin_indices.clear();
    }
}
