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
	text: 'Shanel Atieno, a longstanding member of Otacho Christian Children\'s Home, shares a profound journey enriched by the transformative programs offered. Having spent a significant part of her life here, Shanel attests to the incredible impact of the home\'s initiatives. Through unwavering support, education, and a nurturing environment, she has not only found stability but thrived. Shanel\'s success is a testament to the effectiveness of the programs and the dedication of those behind Otacho Christian Children\'s Home. Your support contributes to creating success stories like Shanel\'s, fostering lasting positive change',
	name: 'Shanel Atieno',
	color: 'rgba(250, 130, 49,1.0)'
},{
	text: 'Jerusa Achieng, a resilient spirit, has called Otacho Christian Children\'s Home her haven for the past decade. Now, on the brink of sitting for her national exams, Jerusa\'s journey is a testament to the transformative power of this supportive community. The home\'s commitment to education, care, and empowerment has fueled her aspirations. Your support for Otacho Christian Children\'s Home contributes to success stories like Jerusa\'s, empowering children to overcome challenges and excel in their pursuits. Join us in championing the dreams of children who deserve a chance to succeed',
	name: 'Jerusa Achieng',
	color: 'rgba(75, 123, 236,1.0)'
},{
	text: 'Gift Akinyi, born into adversity with the passing of her father and the tragic loss of her mother during childbirth, found solace at Otacho Christian Children\'s Home. At just 3 years old, Gift has been embraced by the loving community that provides her with care, support, and hope for the future. Your support for Otacho Christian Children\'s Home helps create a nurturing environment where children like Gift can thrive despite early challenges. Join us in making a difference for Gift and others, offering them a chance for a brighter tomorrow.',
	name: 'Gift Akinyi',
	color: '#1dd1a1'
},{
	text: 'Stallon Mark, a resilient 5-year-old, faced early challenges with the loss of his grandmother and unknown parentage. For the past four years, Otacho Christian Children\'s Home has been his refuge, providing love, care, and a nurturing environment. Stallon, despite his young age, embodies the positive impact of the home\'s support. Your contributions to Otacho Christian Children\'s Home help children like Stallon build a foundation of security and hope for a brighter future. Join us in making a difference in the lives of children who deserve a chance to thrive',
	name: 'Stallon Mark',
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