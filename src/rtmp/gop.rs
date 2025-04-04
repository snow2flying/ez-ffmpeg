use std::collections::VecDeque;
use bytes::Bytes;
use rml_rtmp::time::RtmpTimestamp;

#[derive(Clone)]
pub(crate) enum FrameData {
    Video {
        timestamp: RtmpTimestamp,
        data: Bytes,
    },
    Audio {
        timestamp: RtmpTimestamp,
        data: Bytes,
    },
}

#[derive(Clone)]
pub(crate) struct Gop {
    datas: Vec<FrameData>,
}

impl Default for Gop {
    fn default() -> Self {
        Self::new()
    }
}

impl Gop {
    pub fn new() -> Self {
        Self { datas: Vec::new() }
    }

    fn save_frame_data(&mut self, data: FrameData) {
        self.datas.push(data);
    }

    pub fn get_frame_data(self) -> Vec<FrameData> {
        self.datas
    }

    pub fn len(&self) -> usize {
        self.datas.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone)]
pub struct Gops {
    gops: VecDeque<Gop>,
    size: usize,
}

impl Default for Gops {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Gops {
    pub fn new(size: usize) -> Self {
        Self {
            gops: VecDeque::from([Gop::new()]),
            size,
        }
    }

    pub fn save_frame_data(&mut self, data: FrameData, is_key_frame: bool) {
        if self.size == 0 {
            return;
        }

        if is_key_frame {
            if self.gops.len() == self.size {
                self.gops.pop_front();
            }
            self.gops.push_back(Gop::new());
        }

        if let Some(gop) = self.gops.back_mut() {
            gop.save_frame_data(data);
        } else {
            log::error!("should not be here!");
        }
    }

    pub fn setted(&self) -> bool {
        self.size != 0
    }

    pub fn get_gops(&self) -> VecDeque<Gop> {
        self.gops.clone()
    }
}
