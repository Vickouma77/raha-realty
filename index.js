$(document).ready(function(){
    $('.hm-menu').click(function(){
        $('header').toggleClass('h-100');
        $('.hm-menu span').toggleClass('hm-100');
        $('html').toggleClass('over-x');
    });
     
     $('header nav a').click(function(){
        $('header').removeClass('h-100');
        $('.hm-menu span').removeClass('hm-100');
         $('html').removeClass('over-x');
    });
     
 });

 const authorsEl = document.querySelectorAll('.author');
const container = document.querySelector('.testimonials-container');
const nameEl = document.querySelector('.name');
const textEl = document.querySelector('.text');

const testimonials = [{
	text: 'Cynthia Atieno, orphaned at a young age, found solace and family at Otacho Christian Children\'s Home. This haven provided not just shelter but love, education, and a supportive community. The nurturing environment transformed her life, offering hope and opportunities. Cynthia, now on the brink of a promising future, expresses gratitude to the caregivers, staff, and supporters who believed in her potential. Your contribution to Otacho Christian Children\'s Home goes beyond a building—it plants seeds of hope, changes lives, and empowers children to dream.',
	name: 'Cynthia Atieno',
	color: '#feca57'
},{
	text: '',
	name: 'Joy clara',
	color: 'rgba(250, 130, 49,1.0)'
},{
	text: 'Alexandru Florin never fails to impress me by the quality of his work and delivering on time. When it comes to front-end, he is definitely my go to guy. Always is a pleasure to work with Alexandru even when deadlines are tight and pressure is great. It\'s reassuring to have a project in such good hands.',
	name: 'Otonglo Jaduong',
	color: 'rgba(75, 123, 236,1.0)'
},{
	text: 'Florin has been of great help on our different web projects. He is very trustworthy and serious in the work done. Keep on the good work and energy, been a pleasure to collaborate.',
	name: 'Mulili Mwala',
	color: '#1dd1a1'
},{
	text: 'I hired Florin Pop based on others positive experiences, and I understand why he\'s so highly rated. His code is very clean, he communicates well, and he likes to offer alternative solutions.',
	name: 'Kanye South',
	color: '#ff2828'
}];

addTestimonial(0);

authorsEl.forEach((author, idx) => {
	author.addEventListener('click', (e) => {
		addTestimonial(idx);
		author.classList.add('selected');
	})
});

function addTestimonial(idx) {
	const testimonial = testimonials[idx];
	
	nameEl.innerHTML = testimonial.name;
	textEl.innerHTML = testimonial.text;
	container.style.background = testimonial.color;
	container.style.boxShadow = `0 35px 10px -20px ${testimonial.color.substring(0, testimonial.color.length-4)}0.9)`;
	
	authorsEl.forEach(author => {
		author.classList.remove('selected');
	});
}

// SOCIAL PANEL JS
const floating_btn = document.querySelector('.floating-btn');
const close_btn = document.querySelector('.close-btn');
const social_panel_container = document.querySelector('.social-panel-container');

floating_btn.addEventListener('click', () => {
	social_panel_container.classList.toggle('visible')
});

close_btn.addEventListener('click', () => {
	social_panel_container.classList.remove('visible')
});