mod adapter_state;
mod backend_state;
mod buffer_state;
mod descriptor_set;
mod device_state;
mod framebuffer_state;
mod pipeline_state;
mod render_pass_state;
mod renderer_state;
mod swapchain_state;
mod uniform;

use hal::Backend;

pub use self::renderer_state::RendererState;

use self::adapter_state::AdapterState;
use self::backend_state::BackendState;
use self::buffer_state::BufferState;
use self::descriptor_set::{DescSet, DescSetLayout, DescSetWrite};
use self::device_state::DeviceState;
use self::framebuffer_state::FramebufferState;
use self::pipeline_state::PipelineState;
use self::render_pass_state::RenderPassState;
use self::swapchain_state::SwapchainState;
use self::uniform::Uniform;

type BackendImpl = back::Backend;
type BufferImpl = <BackendImpl as Backend>::Buffer;
type DeviceImpl = <BackendImpl as Backend>::Device;
type DescriptorSetImpl = <BackendImpl as Backend>::DescriptorSet;
type DescriptorSetLayoutImpl = <BackendImpl as Backend>::DescriptorSetLayout;
type DescriptorPoolImpl = <BackendImpl as Backend>::DescriptorPool;
type ImageImpl = <BackendImpl as Backend>::Image;
type ImageViewImpl = <BackendImpl as Backend>::ImageView;
type GraphicsPipelineImpl = <BackendImpl as Backend>::GraphicsPipeline;
type FenceImpl = <BackendImpl as Backend>::Fence;
type FramebufferImpl = <BackendImpl as Backend>::Framebuffer;
type SemaphoreImpl = <BackendImpl as Backend>::Semaphore;
type SurfaceImpl = <BackendImpl as Backend>::Surface;
type SwapchainImpl = <BackendImpl as Backend>::Swapchain;
type RenderPassImpl = <BackendImpl as Backend>::RenderPass;
type MemoryImpl = <BackendImpl as Backend>::Memory;
type PhysicalDeviceImpl = <BackendImpl as Backend>::PhysicalDevice;
type PipelineLayoutImpl = <BackendImpl as Backend>::PipelineLayout;
